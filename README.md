# 萤石API

## 简介
基于萤石open API，获取摄像头数据，包括：

* 设备列表
* 设备信息（含电量）
* 告警信息（支持告警关联图片解码）

## 使用

1. 首先去[萤石开放平台](https://open.ys7.com/ )申请开发者账号，新建应用，获取：
    * AppKey
    * Secret
2. 编译代码：`cargo build`
3. 执行

执行流程：

* 配置秘钥：`cargo run config --key ${AppKey} --secret ${Secret}`
* 列出设备：`cargo run device list`
* 设置默认设备：`cargo run config --default-device ${device_serial}`
* 查看设备信息：`cargo run device info`
* 查看最近告警信息：`cargo run alarm --start "2020-11-12 00:00:00" --end "2020-11-12 23:59:59"`。其中alarm_type为告警类型，具体见：https://open.ys7.com/doc/zh/book/index/alarmType.html#alarm_type 
* 解码图片：目前不支持直接解码，对于告警信息中is_encrypt为1的，表示图片为加密图片，需要下载alarm_pic_url中的文件，然后再运行：`cargo run decrypt -f ${src} -c ${CODE}`。解密之后会生成一个jpg文件在当前文件夹。其中CODE为加密秘钥，默认为设备验证码，可以在设备上查看。改值可以在app中修改。