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

struct DownloadProgressReceive : public zypp::callback::ReceiveReport<zypp::media::DownloadProgressReport>
{
  int last_reported;
  time_t last_reported_time;
  ZyppDownloadStartCallback start_callback;
  ZyppDownloadProgressCallback progress_callback;
  ZyppDownloadProblemCallback problem_callback;
  ZyppDownloadFinishCallback finish_callback;
  void *user_data;

  void set_callbacks(ZyppDownloadStartCallback start,
                     ZyppDownloadProgressCallback progress,
                     ZyppDownloadProblemCallback problem,
                     ZyppDownloadFinishCallback finish, void *user_data_)
  {
    start_callback = start;
    progress_callback = progress;
    problem_callback = problem;
    finish_callback = finish;
    user_data = user_data_;
  }

  virtual void start(const zypp::Url &file, zypp::Pathname localfile)
  {
    last_reported = 0;
    last_reported_time = time(NULL);

    if (start_callback != NULL)
    {
      start_callback(file.asString().c_str(), localfile.c_str(), user_data);
    }
  }

  virtual bool progress(int value, const zypp::Url &file, double bps_avg, double bps_current)
  {
    // call the callback function only if the difference since the last call is at least 5%
    // or if 100% is reached or if at least 3 seconds have elapsed
    time_t current_time = time(NULL);
    const int timeout = 3;
    if (progress_callback != NULL && (value - last_reported >= 5 || last_reported - value >= 5 || value == 100 || current_time - last_reported_time >= timeout))
    {
      last_reported = value;
      last_reported_time = current_time;
      // report changed values
      return progress_callback(value, file.asString().c_str(), bps_avg, bps_current, user_data) != 0;
    }

    return true;
  }

  virtual Action problem(const zypp::Url &file, zypp::media::DownloadProgressReport::Error error, const std::string &description)
  {
    if (problem_callback != NULL)
    {
      PROBLEM_RESPONSE response = problem_callback(file.asString().c_str(), error, description.c_str(), user_data);

      switch (response)
      {
      case PROBLEM_RETRY:
        return zypp::media::DownloadProgressReport::RETRY;
      case PROBLEM_ABORT:
        return zypp::media::DownloadProgressReport::ABORT;
      case PROBLEM_IGNORE:
        return zypp::media::DownloadProgressReport::IGNORE;
      }
    }
    // otherwise return the default value from the parent class
    return zypp::media::DownloadProgressReport::problem(file, error, description);
  }

  virtual void finish(const zypp::Url &file, zypp::media::DownloadProgressReport::Error error, const std::string &reason)
  {
    if (finish_callback != NULL)
    {
      finish_callback(file.asString().c_str(), error, reason.c_str(), user_data);
    }
  }
};

static DownloadProgressReceive download_progress_receive;

extern "C"
{
  void set_zypp_progress_callback(ZyppProgressCallback progress, void *user_data)
  {
    progress_receive.set_callback(progress, user_data);
    progress_receive.connect();
  }

  void set_zypp_download_callbacks(ZyppDownloadStartCallback start,
    ZyppDownloadProgressCallback progress, ZyppDownloadProblemCallback problem,
    ZyppDownloadFinishCallback finish, void *user_data) {
      download_progress_receive.set_callbacks(start, progress, problem, finish, user_data);
      download_progress_receive.connect();
    }
}

#ifdef __cplusplus
zypp::ProgressData::ReceiverFnc get_progress_callback()
{
  zypp::ProgressData::ReceiverFnc progress_handler(boost::bind(&ProgressReceive::progress, &progress_receive, _1));
  return progress_handler;
}

bool dynamic_progress_callback(ZyppProgressCallback progress, void *user_data, const zypp::ProgressData &task)
{
  printf("progress called\n");
  if (progress != NULL)
  {
    printf("own callback defined\n");
    ProgressData data = {task.reportValue(), task.name().c_str()};
    return progress(data, user_data) != 0;
  }
  else
  {
    return true;
  }
}

zypp::ProgressData::ReceiverFnc create_progress_callback(ZyppProgressCallback progress, void *user_data)
{
  return zypp::ProgressData::ReceiverFnc(boost::bind(dynamic_progress_callback, progress, user_data, _1));
}
#endif
