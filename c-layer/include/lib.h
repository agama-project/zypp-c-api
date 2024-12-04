#ifndef C_LIB_H_
#define C_LIB_H_

#include "callbacks.h"

#ifdef __cplusplus
extern "C" {
#endif
#ifndef __cplusplus
#define noexcept ;
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
void free_status(struct Status *s) noexcept;

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
void init_target(const char *root, struct Status *status, ProgressCallback progress, void *user_data) noexcept;

/// repository array in list.
/// when no longer needed, use \ref free_repository_list to release memory
/// @param[out] status (will overwrite existing contents)
struct RepositoryList list_repositories(struct Status *status) noexcept;

void free_repository_list(struct RepositoryList *repo_list) noexcept;

/// Adds repository to repo manager
/// @param alias have to be unique
/// @param url can contain repo variables
/// @param[out] status (will overwrite existing contents)
/// @param callback pointer to function with callback or NULL
/// @param user_data
void add_repository(const char* alias, const char* url, struct Status *status, ZyppProgressCallback callback, void* user_data) noexcept;

/// Removes repository from repo manager
/// @param alias have to be unique
/// @param[out] status (will overwrite existing contents)
/// @param callback pointer to function with callback or NULL
/// @param user_data
void remove_repository(const char* alias, struct Status *status, ZyppProgressCallback callback, void* user_data) noexcept;

///
/// @param alias alias of repository to refresh
/// @param[out] status (will overwrite existing contents)
/// @param callbacks pointer to struct with callbacks or NULL if no progress is needed
void refresh_repository(const char* alias, struct Status* status, struct DownloadProgressCallbacks *callbacks) noexcept;


enum RESOLVABLE_KIND {
  RESOLVABLE_PRODUCT,
  RESOLVABLE_PATCH,
  RESOLVABLE_PACKAGE,
  RESOLVABLE_SRCPACKAGE,
  RESOLVABLE_PATTERN,
};

/// Marks resolvable for installation
/// @param name resolvable name
/// @param kind kind of resolvable
/// @param[out] status (will overwrite existing contents)
void resolvable_select(const char* name, enum RESOLVABLE_KIND kind, struct Status* status) noexcept;

/// Unselect resolvable for installation. It can still be installed as dependency.
/// @param name resolvable name
/// @param kind kind of resolvable
/// @param[out] status (will overwrite existing contents)
void resolvable_unselect(const char* name, enum RESOLVABLE_KIND kind, struct Status* status) noexcept;

// the last call that will free all pointers to zypp holded by agama
void free_zypp() noexcept;

#ifdef __cplusplus
}
#endif
#endif
