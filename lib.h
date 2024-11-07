#ifndef C_LIB_H_
#define C_LIB_H_

#include <stdlib.h>

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

  void free_repository_list(struct RepositoryList* repo_list);

  int init_target(const char* root);

  // repository array in list is by caller
  // when no longer needed, use free_repository_list
  struct RepositoryList list_repositories();

  // the last call that will free all pointers to zypp holded by agama
  void free_zypp();

#ifdef __cplusplus
}
#endif
#endif
