#include "callbacks.h"
#include "lib.h"
#include <stdio.h>

void progress(const char *text, unsigned stage, unsigned total, void *data) {
  printf("(%s) %u/%u: %s\n", (const char *)data, stage, total, text);
}

int zypp_progress(struct ProgressData data, void *user_data) {
  printf("(%s) %lld%%\n", data.name, data.value);
  return 1;
}

void download_progress_start(const char *url, const char *localfile, void *user_data) {
  printf("Starting download of %s to %s\n", url, localfile);
}

int download_progress_progress(int value, const char *url, double bps_avg, double bps_current, void *user_data) {
  printf("Downloading %s with %i%% (speed: now %f avg %f)\n", url, value, bps_current, bps_avg);
  return 1;
}

enum PROBLEM_RESPONSE download_progress_problem(const char *url, int error, const char *description, void *user_data) {
  printf("Download ERROR for %s: %s\n", url, description);
  printf("Aborting...\n");
  return PROBLEM_ABORT;
}

void download_progress_finish(const char *url, int error, const char *reason, void *user_data) {
  printf("Download of %s finished with %s\n", url, reason);
}

int main(int argc, char *argv[]) {
  int result = 0;
  struct Status status;
  struct DownloadProgressCallbacks download_callbacks = {
    download_progress_start, download_progress_progress, download_progress_problem, download_progress_finish, NULL
  };

  set_zypp_progress_callback(zypp_progress, NULL);
  set_zypp_download_callbacks(download_callbacks);

  char * root = "/";
  if (argc > 1)
    root = argv[1];
  printf("List of repos:\n");
  const char *prefix = "Loading '/'"; // TODO: wrong report with changed root
  init_target(root, &status, progress, (void *)prefix);
  if (status.state != STATE_SUCCEED) {
    printf("init ERROR!: %s\n", status.error);
    result = 1;
    goto norepo;
  }
  free_status(&status);

  struct RepositoryList list = list_repositories();
  for (unsigned i = 0; i < list.size; ++i) {
    struct Repository *repo = list.repos + i;
    printf("repo %i: %s\n", i, repo->userName);
    printf("refreshing...");
    refresh_repository(repo->alias, &status);
    if (status.state != STATE_SUCCEED) {
      printf("refresh ERROR!: %s\n", status.error);
      free_status(&status);
      goto repoerr;
   }
   free_status(&status);
  }

repoerr:
  free_repository_list(&list);
norepo:
  free_status(&status);
  free_zypp();
  return result;
}
