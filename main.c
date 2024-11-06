#include <stdio.h>
#include "lib.h"

int main() {
   printf("List of repos:\n");
   init_target("/");
   struct RepositoryList list = list_repositories();
   for (unsigned i = 0; i < list.size; ++i){
      struct Repository* repo = list.repos + i;
      printf("repo %i: %s\n", i, repo->userName);
   }
   return 0;
}
