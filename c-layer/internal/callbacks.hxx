#ifndef C_CALLBACKS_HXX_
#define C_CALLBACKS_HXX_

// C++ specific code call that cannot be used from C. Used to pass progress class between o files.
#include <zypp-core/ui/progressdata.h>
zypp::ProgressData::ReceiverFnc get_progress_callback();
zypp::ProgressData::ReceiverFnc create_progress_callback(ZyppProgressCallback progress, void *user_data);

#endif
