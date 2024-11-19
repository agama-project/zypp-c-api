#ifndef C_LIB_H_
#define C_LIB_H_

#include "callbacks.h"

#ifdef __cplusplus
extern "C" {
#endif
struct Repository {
  char *url;        ///< owned
  char *alias;      ///< owned
  char *userName;   ///< owned
};

struct RepositoryList {
  const unsigned size;
  /// dynamic array with given size
  struct Repository *repos; ///< owned, *size* items
};

/// status struct to pass and obtain from calls that can fail.
/// After usage free with \ref free_status function.
///
/// Most functions act as *constructors* for this, taking a pointer
/// to it as an output parameter, disregarding the struct current contents
/// and filling it in. Thus, if you reuse a `Status` without \ref free_status
/// in between, `error` will leak.
struct Status {
  // lets use enum for future better distinguish
  enum STATE {
    STATE_SUCCEED,
    STATE_FAILED,
  } state;
  /// detailed user error what happens. Only defined when not succeed
  char *error; ///< owned
};
void free_status(struct Status s);

/// Progress reporting callback used by methods that takes longer.
/// @param text  text for user describing what is happening now
/// @param stage current stage number starting with 0
/// @param total count of stages. It should not change during single call of method.
/// @param user_data is never touched by method and is used only to pass local data for callback
/// @todo Do we want to support response for callback that allows early exit of execution?
typedef void (*ProgressCallback)(const char *text, unsigned stage, unsigned total, void *user_data);
/// Initialize Zypp target (where to install packages to)
/// @param root
/// @param[out] status
/// @param progress
/// @param user_data
void init_target(const char *root, struct Status *status, ProgressCallback progress, void *user_data);

/// repository array in list.
/// when no longer needed, use \ref free_repository_list to release memory
struct RepositoryList list_repositories();

void free_repository_list(struct RepositoryList *repo_list);

///
/// @param alias alias of repository to refresh
/// @param[out] status (will overwrite existing contents)
void refresh_repository(const char* alias, struct Status* status);

// the last call that will free all pointers to zypp holded by agama
void free_zypp();

#ifdef __cplusplus
}
#endif
#endif
