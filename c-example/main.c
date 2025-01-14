#include "callbacks.h"
#include "lib.h"
#include "repository.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

void progress(const char *text, unsigned stage, unsigned total, void *data) {
  printf("(%s) %u/%u: %s\n", (const char *)data, stage, total, text);
}

bool zypp_progress(struct ProgressData data, void *user_data) {
  printf("(%s) %lld%%\n", data.name, data.value);
  return true;
}

void download_progress_start(const char *url, const char *localfile,
                             void *user_data) {
  printf("Starting download of %s to %s\n", url, localfile);
}

bool download_progress_progress(int value, const char *url, double bps_avg,
                                double bps_current, void *user_data) {
  printf("Downloading %s with %i%% (speed: now %f avg %f)\n", url, value,
         bps_current, bps_avg);
  return true;
}

enum PROBLEM_RESPONSE download_progress_problem(const char *url, int error,
                                                const char *description,
                                                void *user_data) {
  printf("Download ERROR for %s: %s\n", url, description);
  printf("Aborting...\n");
  return PROBLEM_ABORT;
}

void download_progress_finish(const char *url, int error, const char *reason,
                              void *user_data) {
  printf("Download of %s finished with status %d (%s)\n", url, error, reason);
}

int main(int argc, char *argv[]) {
  int result = EXIT_SUCCESS;
  struct Status status;
  struct DownloadProgressCallbacks download_callbacks = {
      download_progress_start,   NULL, download_progress_progress, NULL,
      download_progress_problem, NULL, download_progress_finish,   NULL};

  char *root = "/";
  if (argc > 1)
    root = argv[1];
  printf("List of repos:\n");
  const char *prefix = "Loading '/'"; // TODO: wrong report with changed root
  struct Zypp *zypp = init_target(root, &status, progress, (void *)prefix);
  if (status.state != STATE_SUCCEED) {
    printf("init ERROR!: %s\n", status.error);
    result = EXIT_FAILURE;
    goto nozypp;
  }
  free_status(&status);

  printf("Existing repos:");
  struct RepositoryList list = list_repositories(zypp, &status);
  if (status.state != STATE_SUCCEED) {
    printf("list_repositories ERROR!: %s\n", status.error);
    result = EXIT_FAILURE;
    goto norepo;
  }
  free_status(&status);

  for (unsigned i = 0; i < list.size; ++i) {
    struct Repository *repo = list.repos + i;
    printf("repo %i: %s\n", i, repo->userName);
  }

  printf("\n\n");
  printf("Adding new repo with Agama Devel\n");
  add_repository(zypp, "agama",
                 "https://download.opensuse.org/repositories/"
                 "systemsmanagement:/Agama:/Devel/openSUSE_Tumbleweed/",
                 &status, zypp_progress, NULL);
  if (status.state != STATE_SUCCEED) {
    printf("failed to add repo!: %s\n", status.error);
    result = EXIT_FAILURE;
    goto repoerr;
  }
  free_status(&status);
  printf("Refreshing it");
  refresh_repository(zypp, "agama", &status, &download_callbacks);
  if (status.state != STATE_SUCCEED) {
    printf("refresh ERROR!: %s\n", status.error);
    goto repoerr;
  }

repoerr:
  free_repository_list(&list);
norepo:
  free_zypp(zypp);
nozypp:
  free_status(&status);
  return result;
}
