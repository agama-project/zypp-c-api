#include <zypp/Callback.h>
#include <zypp/ZYppCallbacks.h>

#include "callbacks.h"

extern "C" {
  struct ProgressReceive : zypp::callback::ReceiveReport<zypp::ProgressReport>
  {
      ZyppProgressCallback callback;
      void *user_data;

      ProgressReceive() {}

      void set_callback(ZyppProgressCallback callback_, void *user_data_) {
        callback = callback_;
        user_data = user_data_;
      }

    // TODO: should we distinguish start/finish? and if so, is enum param to callback enough instead of having three callbacks?
      virtual void start(const zypp::ProgressData &task)
      {
          if (callback != NULL) {
            ProgressData data = { task.reportValue(), task.name().c_str() };
            callback(data, user_data);
          }
      }

      virtual bool progress(const zypp::ProgressData &task)
      {
          if (callback != NULL) {
            ProgressData data = { task.reportValue(), task.name().c_str() };
            return callback(data, user_data) != 0;
          } else {
            return zypp::ProgressReport::progress(task);
          } 
      }

      virtual void finish( const zypp::ProgressData &task )
      {   
          if (callback != NULL) {
            ProgressData data = { task.reportValue(), task.name().c_str() };
            callback(data, user_data);
          }
      }
  };

  static ProgressReceive progress_receive;

  void set_zypp_progress_callback (ZyppProgressCallback progress, void *user_data) {
    progress_receive.set_callback(progress, user_data);
    progress_receive.connect();
  }
}

