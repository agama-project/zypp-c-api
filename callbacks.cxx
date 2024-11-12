#include <zypp/Callback.h>
#include <zypp/ZYppCallbacks.h>
#include <boost/bind.hpp>

#include "callbacks.h"

struct ProgressReceive : zypp::callback::ReceiveReport<zypp::ProgressReport>
{
  ZyppProgressCallback callback;
  void *user_data;

  ProgressReceive() {}

  void set_callback(ZyppProgressCallback callback_, void *user_data_)
  {
    callback = callback_;
    user_data = user_data_;
    printf("Callback assigned\n");
  }

  // TODO: should we distinguish start/finish? and if so, is enum param to callback enough instead of having three callbacks?
  virtual void start(const zypp::ProgressData &task)
  {
    if (callback != NULL)
    {
      ProgressData data = {task.reportValue(), task.name().c_str()};
      callback(data, user_data);
    }
  }

  bool progress(const zypp::ProgressData &task)
  {
    printf("progress called\n");
    if (callback != NULL)
    {
      printf("own callback defined\n");
      ProgressData data = {task.reportValue(), task.name().c_str()};
      return callback(data, user_data) != 0;
    }
    else
    {
      return zypp::ProgressReport::progress(task);
    }
  }

  virtual void finish(const zypp::ProgressData &task)
  {
    if (callback != NULL)
    {
      ProgressData data = {task.reportValue(), task.name().c_str()};
      callback(data, user_data);
    }
  }
};

static ProgressReceive progress_receive;

extern "C"
{
  void set_zypp_progress_callback(ZyppProgressCallback progress, void *user_data)
  {
    progress_receive.set_callback(progress, user_data);
    progress_receive.connect();
  }
}

#ifdef __cplusplus
zypp::ProgressData::ReceiverFnc get_progress_callback()
{
  zypp::ProgressData::ReceiverFnc progress_handler(boost::bind(&ProgressReceive::progress, &progress_receive, _1));
  return progress_handler;
}

bool dynamic_progress_callback(ZyppProgressCallback progress, void *user_data, const zypp::ProgressData &task){
    printf("progress called\n");
    if (progress != NULL)
    {
      printf("own callback defined\n");
      ProgressData data = {task.reportValue(), task.name().c_str()};
      return progress(data, user_data) != 0;
    } else {
      return true;
    }
}

zypp::ProgressData::ReceiverFnc create_progress_callback(ZyppProgressCallback progress, void *user_data) {
  return zypp::ProgressData::ReceiverFnc(boost::bind(dynamic_progress_callback, progress, user_data, _1));
}
#endif
