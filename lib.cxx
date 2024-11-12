#include "lib.h"
#include "callbacks.h"

#include <zypp/ZYppFactory.h>
#include <zypp/ZYpp.h>
#include <zypp/RepoManager.h>

extern "C" {
  static zypp::ZYpp::Ptr zypp_pointer = NULL;
  static zypp::RepoManager* repo_manager = NULL;

  void free_zypp() {
    zypp_pointer = NULL; // shared ptr assignment operator will free original pointer
    delete(repo_manager);
  }

  zypp::ZYpp::Ptr zypp_ptr() {
    if (zypp_pointer != NULL)
    {
	return zypp_pointer;
    }

    int max_count = 5;
    unsigned int seconds = 3;

    while (zypp_pointer == NULL && max_count > 0)
    {
	try
	{
	    zypp_pointer = zypp::getZYpp();

	    return zypp_pointer;
	}
	catch (const zypp::Exception &excpt)
	{
          max_count--;

	  sleep(seconds);
	}
    }

    return NULL;
  }

  int init_target(const char* root, ProgressCallback progress, void *user_data) {
    const std::string root_str(root);

    try
    {
        zypp::RepoManagerOptions repo_manager_options(root);
        // repository manager options cannot be replaced, a new repository manager is needed
        zypp::RepoManager* new_repo_manager = new zypp::RepoManager(repo_manager_options);

        // replace the old repository manager
        if (repo_manager) delete repo_manager;
        repo_manager = new_repo_manager;

        // TODO: localization
        progress("Initializing the Target System", 0, 2, user_data);
	zypp_ptr()->initializeTarget(root_str, false);
        progress("Reading Installed Packages", 1, 2, user_data);
        zypp_ptr()->target()->load();
    }
    catch (zypp::Exception & excpt)
    {
        return 0;
    }

    return 1;
  }

  void free_repository(struct Repository* repo) {
    free(repo->url);
    free(repo->alias);
    free(repo->userName);
  }

  void free_repository_list(struct RepositoryList* list) {
    for (unsigned i = 0; i < list->size; ++i) {
      free_repository(list->repos+i);
    }
    free(list->repos);
  }

  void free_status(struct Status status) {
    if (status.error != NULL) {
      free(status.error);
    }
  }

  void refresh_repositories(struct Status* status, ZyppProgressCallback callback, void *data) {
    if (repo_manager == NULL) {
      status->state = status->STATE_FAILED;
      status->error = strdup("Internal Error: Repo manager is not initialized.");
      return;
    }

    auto progress_cb = create_progress_callback(callback, data);

    try {
      std::list<zypp::RepoInfo> zypp_repos = repo_manager->knownRepositories();
      for (auto iter = zypp_repos.begin(); iter != zypp_repos.end(); ++iter) {
        repo_manager->refreshMetadata(*iter, zypp::RepoManager::RawMetadataRefreshPolicy::RefreshForced, progress_cb);
      }
    } catch (zypp::Exception & excpt)
    {
      status->state = status->STATE_FAILED;
      status->error = strdup(excpt.asUserString().c_str());
    }
  }

  struct RepositoryList list_repositories() {
    if (repo_manager == NULL) {
      //TODO: error reporting?
      return { 0, NULL };
    }

    std::list<zypp::RepoInfo> zypp_repos = repo_manager->knownRepositories();
    const std::list<zypp::RepoInfo>::size_type size = zypp_repos.size();
    struct Repository* repos = (struct Repository*) malloc(size * sizeof(struct Repository));
    // TODO: error handling
    unsigned res_i = 0;
    for (auto iter = zypp_repos.begin(); iter != zypp_repos.end(); ++iter) {
      struct Repository* new_repo = repos + res_i++;
      new_repo->url = strdup(iter->url().asString().c_str());
      new_repo->alias = strdup(iter->alias().c_str());
      new_repo->userName = strdup(iter->asUserString().c_str());
    }

    struct RepositoryList result = { static_cast<unsigned>(size), repos };
    return result;
  }
}
