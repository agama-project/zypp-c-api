#include <stdio.h>
#include "lib.h"
#include "callbacks.h"

void progress(const char* text, unsigned stage, unsigned total, void *data) {
   printf("(%s) %u/%u: %s\n", (const char*) data, stage, total, text);
}

int zypp_progress(struct ProgressData data, void *user_data) {
   printf("(%s) %lld%%\n", data.name, data.value);
   return 1;
}

int main() {
   set_zypp_progress_callback(zypp_progress, NULL);
   printf("List of repos:\n");
   const char* prefix = "Loading '/'";
   init_target("/", progress, (void *)prefix);
   struct RepositoryList list = list_repositories();
   for (unsigned i = 0; i < list.size; ++i){
      struct Repository* repo = list.repos + i;
      printf("repo %i: %s\n", i, repo->userName);
   }
   free_repository_list(&list);

   // refresh all repos to get some zypp progress
   struct Status status;
   refresh_repositories(&status, zypp_progress, NULL);
   if (status.state != STATE_SUCCEED) {
      printf("refresh ERROR!: %s\n", status.error);
   }
   free_status(status);
   free_zypp();
   return 0;
}
