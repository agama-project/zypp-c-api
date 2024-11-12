#ifndef C_LIB_H_
#define C_LIB_H_

#include "callbacks.h"

#ifdef __cplusplus
extern "C" {
#endif
  struct Repository {
    char* url;
    char* alias;
    char* userName;
  };

  struct RepositoryList {
    const unsigned size;
    // dynamic array with given size
    struct Repository* repos;
  };

  // status struct to pass and obtain from calls that can fail.
  // After usage free with free_status function
  struct Status {
    // lets use enum for future better distinguish
    enum STATE {
      STATE_SUCCEED,
      STATE_FAILED,
    } state;
    // detailed user error what happens. Only defined when not succeed
    char * error;
  };
  void free_status (struct Status s);

  // Progress reporting callback used by methods that takes longer.
  // text is text for user describing what is happening now
  // stage is current stage number starting with 0
  // total is count of stages. It should not change during single call of method.
  // user_data is never touched by method and is used only to pass local data for callback
  // TODO: Do we want to support response for callback that allows early exit of execution?
  typedef void (*ProgressCallback)(const char *text, unsigned stage, unsigned total, void *user_data);
  int init_target(const char* root, ProgressCallback progress, void *user_data);

  // repository array in list
  // when no longer needed, use free_repository_list to release memory
  struct RepositoryList list_repositories();

  void free_repository_list(struct RepositoryList* repo_list);

  void refresh_repositories(struct Status* status, ZyppProgressCallback callback, void *data);

  // the last call that will free all pointers to zypp holded by agama
  void free_zypp();

#ifdef __cplusplus
}
#endif
#endif
