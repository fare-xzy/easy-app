import { notification, message } from 'ant-design-vue'
import moment from 'moment'
import { App } from 'vue'

const $timeFix = (): string => {
  const time = new Date()
  const hour = time.getHours()
  return hour < 9
    ? '早上好'
    : hour <= 11
    ? '上午好'
    : hour <= 13
    ? '中午好'
    : hour < 20
    ? '下午好'
    : '晚上好'
}

// 获取当前时间
const $getTime = (): string => {
  //	hh12小时制
  // HH为24小时制
  return moment(new Date()).format('YYYY-MM-DD HH:mm:ss')
}

export default {
  install: (app: App): void => {
    app.config.globalProperties['$timeFix'] = $timeFix
    app.config.globalProperties['$getTime'] = $getTime
    app.config.globalProperties.$notification = notification
    app.config.globalProperties.$message = message
  }
}
