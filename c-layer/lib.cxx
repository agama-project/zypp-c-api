#include "lib.h"
#include "callbacks.h"
#include "callbacks.hxx"

#include <cstddef>
#include <zypp-core/Url.h>
#include <zypp/RepoInfo.h>
#include <zypp/RepoManager.h>
#include <zypp/ResStatus.h>
#include <zypp/Resolvable.h>
#include <zypp/ZYpp.h>
#include <zypp/ZYppFactory.h>
#include <zypp/base/LogControl.h>
#include <zypp/base/Logger.h>

#include <cstdarg>
#include <zypp/ui/Selectable.h>

extern "C" {
static zypp::ZYpp::Ptr zypp_pointer = NULL;
static zypp::RepoManager *repo_manager = NULL;

void free_zypp() noexcept {
  zypp_pointer =
      NULL; // shared ptr assignment operator will free original pointer
  delete (repo_manager);
}

// helper to get allocated formated string. Sadly C does not provide any portable way to do it.
// if we are ok with GNU or glib then it provides it
static char* format_alloc(const char* const format...) {
  // `vsnprintf()` changes `va_list`'s state, so using it after that is UB.
  // We need the args twice, so it is safer to just get two copies.
  va_list args1;
  va_list args2;
  va_start(args1, format);
  va_start(args2, format);

  // vsnprintf with len 0 just return needed size and add trailing zero.
  size_t needed = 1 + vsnprintf(NULL, 0, format, args1);

  char* buffer = (char *) malloc(needed * sizeof(char));

  vsnprintf(buffer, needed, format, args2);

  va_end(args1);
  va_end(args2);

  return buffer;
}

static zypp::ZYpp::Ptr zypp_ptr() {
  if (zypp_pointer != NULL) {
    return zypp_pointer;
  }

  // set logging to ~/zypp-agama.log for now. For final we need to decide it
  zypp::Pathname home(getenv("HOME"));
  zypp::Pathname log_path = home.cat("zypp-agama.log");
  zypp::base::LogControl::instance().logfile(log_path);

  int max_count = 5;
  unsigned int seconds = 3;

  while (zypp_pointer == NULL && max_count > 0) {
    try {
      zypp_pointer = zypp::getZYpp();

      return zypp_pointer;
    } catch (const zypp::Exception &excpt) {
      max_count--;

      sleep(seconds);
    }
  }

  return NULL;
}

void init_target(const char *root, struct Status *status, ProgressCallback progress, void *user_data) noexcept {
  const std::string root_str(root);

  try {
    zypp::RepoManagerOptions repo_manager_options(root);
    // repository manager options cannot be replaced, a new repository manager is needed
    zypp::RepoManager *new_repo_manager = new zypp::RepoManager(repo_manager_options);

    // replace the old repository manager
    if (repo_manager)
      delete repo_manager;
    repo_manager = new_repo_manager;

    // TODO: localization
    if (progress != NULL)
      progress("Initializing the Target System", 0, 2, user_data);
    zypp_ptr()->initializeTarget(root_str, false);
    if (progress != NULL)
      progress("Reading Installed Packages", 1, 2, user_data);
    zypp_ptr()->target()->load();
  } catch (zypp::Exception &excpt) {
    status->state = status->STATE_FAILED;
    status->error = strdup(excpt.asUserString().c_str());
    return;
  }

  status->state = status->STATE_SUCCEED;
  status->error = NULL;
}

void free_repository(struct Repository *repo) {
  free(repo->url);
  free(repo->alias);
  free(repo->userName);
}

void free_repository_list(struct RepositoryList *list) noexcept {
  for (unsigned i = 0; i < list->size; ++i) {
    free_repository(list->repos + i);
  }
  free(list->repos);
}

void free_status(struct Status *status) noexcept {
  if (status->error != NULL) {
    free(status->error);
    status->error = NULL;
  }
}

static zypp::Resolvable::Kind kind_to_zypp_kind(RESOLVABLE_KIND kind) {
  switch (kind) {
    case RESOLVABLE_PACKAGE: return zypp::Resolvable::Kind::package;
    case RESOLVABLE_SRCPACKAGE: return zypp::Resolvable::Kind::srcpackage;
    case RESOLVABLE_PATTERN: return zypp::Resolvable::Kind::pattern;
    case RESOLVABLE_PRODUCT: return zypp::Resolvable::Kind::product;
    case RESOLVABLE_PATCH: return zypp::Resolvable::Kind::patch;     
  }
  return zypp::Resolvable::Kind::nokind; // should not happen
}

void resolvable_select(const char* name, enum RESOLVABLE_KIND kind, struct Status* status) noexcept {
  zypp::Resolvable::Kind z_kind = kind_to_zypp_kind(kind);
  auto selectable = zypp::ui::Selectable::get(z_kind, name);
  if (!selectable) {
    status->state = status->STATE_FAILED;
    status->error = format_alloc("Failed to find %s with name '%s'", z_kind.c_str(), name);
    return;
  }

  status->state = Status::STATE_SUCCEED;
  status->error = NULL;
  selectable->setToInstall(zypp::ResStatus::APPL_HIGH);
}

void resolvable_unselect(const char* name, enum RESOLVABLE_KIND kind, struct Status* status) noexcept {
  zypp::Resolvable::Kind z_kind = kind_to_zypp_kind(kind);
  auto selectable = zypp::ui::Selectable::get(z_kind, name);
  if (!selectable) {
    status->state = status->STATE_FAILED;
    status->error = format_alloc("Failed to find %s with name '%s'", z_kind.c_str(), name);
    return;
  }

  // it can look a bit strange to unset with higher causer then set, but USER is the highest priority that overwrite even locks
  // so we should be ok in Agama.
  selectable->unset(zypp::ResStatus::USER);
  status->state = Status::STATE_SUCCEED;
  status->error = NULL;
}

int run_solver(struct Status* status) noexcept {
  try {
    status->state = Status::STATE_SUCCEED;
    status->error = NULL;
    return zypp_ptr()->resolver()->resolvePool() ? 1 : 0;
  } catch (zypp::Exception &excpt) {
    status->state = status->STATE_FAILED;
    status->error = strdup(excpt.asUserString().c_str());
    return 0; // do not matter much as status indicate failure
  }
}

void refresh_repository(const char* alias, struct Status *status, struct DownloadProgressCallbacks *callbacks) noexcept {
  if (repo_manager == NULL) {
    status->state = status->STATE_FAILED;
    status->error = strdup("Internal Error: Repo manager is not initialized.");
    return;
  }
  try {
    zypp::RepoInfo zypp_repo = repo_manager->getRepo(alias);
    if (zypp_repo == zypp::RepoInfo::noRepo) {
      status->state = status->STATE_FAILED;
      status->error = format_alloc("Cannot refresh repo with alias %s. Repo not found.", alias);
      return;
    }

    set_zypp_download_callbacks(callbacks);
    repo_manager->refreshMetadata(
          zypp_repo, zypp::RepoManager::RawMetadataRefreshPolicy::RefreshForced);
    status->state = status->STATE_SUCCEED;
    status->error = NULL;
    unset_zypp_download_callbacks();
  } catch (zypp::Exception &excpt) {
    status->state = status->STATE_FAILED;
    status->error = strdup(excpt.asUserString().c_str());
    unset_zypp_download_callbacks(); // TODO: we can add C++ final action helper if it is more common
  }
}

void add_repository(const char* alias, const char* url, struct Status *status, ZyppProgressCallback callback, void* user_data) noexcept {
  if (repo_manager == NULL) {
    status->state = status->STATE_FAILED;
    status->error = strdup("Internal Error: Repo manager is not initialized.");
    return;
  }
  try {
    auto zypp_callback = create_progress_callback(callback, user_data);
    zypp::RepoInfo zypp_repo = zypp::RepoInfo();
    zypp_repo.setBaseUrl(zypp::Url(url));
    zypp_repo.setAlias(alias);
    
    repo_manager->addRepository(zypp_repo,zypp_callback);
    status->state = status->STATE_SUCCEED;
    status->error = NULL;
  } catch (zypp::Exception &excpt) {
    status->state = status->STATE_FAILED;
    status->error = strdup(excpt.asUserString().c_str());
  }
}

void remove_repository(const char* alias, struct Status *status, ZyppProgressCallback callback, void* user_data) noexcept {
  if (repo_manager == NULL) {
    status->state = status->STATE_FAILED;
    status->error = strdup("Internal Error: Repo manager is not initialized.");
    return;
  }
  try {
    auto zypp_callback = create_progress_callback(callback, user_data);
    zypp::RepoInfo zypp_repo = zypp::RepoInfo();
    zypp_repo.setAlias(alias); // alias should be unique, so it should always match correct repo
    
    repo_manager->removeRepository(zypp_repo,zypp_callback);
    status->state = status->STATE_SUCCEED;
    status->error = NULL;
  } catch (zypp::Exception &excpt) {
    status->state = status->STATE_FAILED;
    status->error = strdup(excpt.asUserString().c_str());
  }
}

struct RepositoryList list_repositories(struct Status *status) noexcept {
  if (repo_manager == NULL) {
    status->state = status->STATE_FAILED;
    status->error = strdup("Internal Error: Repo manager is not initialized.");
    return {0, NULL};
  }

  std::list<zypp::RepoInfo> zypp_repos = repo_manager->knownRepositories();
  const std::list<zypp::RepoInfo>::size_type size = zypp_repos.size();
  struct Repository *repos = (struct Repository *)malloc(size * sizeof(struct Repository));
  // TODO: error handling
  unsigned res_i = 0;
  for (auto iter = zypp_repos.begin(); iter != zypp_repos.end(); ++iter) {
    struct Repository *new_repo = repos + res_i++;
    new_repo->url = strdup(iter->url().asString().c_str());
    new_repo->alias = strdup(iter->alias().c_str());
    new_repo->userName = strdup(iter->asUserString().c_str());
  }

  struct RepositoryList result = {static_cast<unsigned>(size), repos};
  status->state = status->STATE_SUCCEED;
  status->error = NULL;
  return result;
}
}
