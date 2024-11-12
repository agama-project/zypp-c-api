#ifndef C_CALLBACKS_H_
#define C_CALLBACKS_H_

#ifdef __cplusplus
extern "C" {
#endif
  struct ProgressData {
    // TODO: zypp also reports min/max so it can be either percent, min/max or just alive progress.
    // Should we expose all of them?
    // progress value is either percent or -1 which means just keep alive progress
    long long value;
    // pointer to progress name. Owned by zypp, so lives only as long as callback
    const char* name;
  };


  // Progress reporting callback passed to libzypp.
  // zypp_data is ProgressData get from zypp
  // user_data is never touched by method and is used only to pass local data for callback
  // return value indicate if zypp should abort operation. Can be ignored
  typedef int (*ZyppProgressCallback)(struct ProgressData zypp_data, void *user_data);
  void set_zypp_progress_callback (ZyppProgressCallback progress, void *user_data);
#ifdef __cplusplus
}
// C++ specific code call that cannot be used from C. Used to pass progress class between o files.
#include <zypp-core/ui/progressdata.h>
zypp::ProgressData::ReceiverFnc get_progress_callback();
zypp::ProgressData::ReceiverFnc create_progress_callback(ZyppProgressCallback progress, void *user_data);
#endif
#endif
