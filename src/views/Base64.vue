<template>
  <div style="background-color: #ececec; padding: 10px; height: 100%;">
    <a-row :gutter="8" style="height: 100%;">
      <a-col :span="12" style="height: 100%;">
        <a-card :bordered="false" style="height: 100%;">
          <template #title>
            转换源格式&nbsp;&nbsp;&nbsp;
            <a-radio-group v-model:value="turnType" name="radioGroup">
              <a-radio value="1">文件</a-radio>
              <a-radio value="2">字符串</a-radio>
            </a-radio-group>
          </template>
          <div v-if="turnType === '1'">
            <a-upload-dragger
                v-model:fileList="fileList"
                name="file"
                :before-upload="beforeUpload"
            >
              <p class="ant-upload-drag-icon">
                <inbox-outlined />
              </p>
              <p class="ant-upload-text">单击或拖动文件到此区域进行上传</p>
              <p class="ant-upload-hint">
                支持单次任意文件类型上传。
              </p>
            </a-upload-dragger>
          </div>
          <div v-if="turnType === '2'">
            <a-textarea v-model:value="inPutStr" placeholder="输入内容（Base64字符串 / 原文字符串）" auto-size/>
          </div>
        </a-card>
      </a-col>
      <a-col :span="12">
        <a-card :bordered="false" style="height: 100%">
          <template #title>
            输出字符串&nbsp;&nbsp;&nbsp;
            <CopyOutlined @click="copyToClipboard"/>
          </template>
          <a-textarea :value="outPutStr" placeholder="输出结果（Base64字符串 / 原文字符串）" auto-size/>
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<script setup lang="ts">
import { UploadProps } from 'ant-design-vue'
import { Buffer } from 'buffer'

// 获取全局配置属性
const instance = getCurrentInstance()
const globalProperties = instance!.appContext.config.globalProperties
// 已上传文件列表
const fileList = ref<UploadProps['fileList']>([])

// 入参字符串
const inPutStr = ref('')
// 转换类型
const turnType = ref('1')

// 判断是否为base64
const isBase64 = (str: string): boolean => {
  const regExp = /^(?:[A-Za-z0-9+\/]{4})*(?:[A-Za-z0-9+\/]{2}==|[A-Za-z0-9+\/]{3}=)?$/;
  return regExp.test(str);
}
const fileData = ref(new Uint8Array());
// 输出字符串
const outPutStr = computed(() => {
  if(turnType.value === '1') {
    if (fileList.value!.length > 0 ){
      return Buffer.from(fileData.value).toString('base64')
    }
  } else {
    if(isBase64(inPutStr.value)) {
      return Buffer.from(inPutStr.value, 'base64').toString()
    }else{
      return Buffer.from(inPutStr.value).toString('base64')
    }
  }
})

// 自定义上传，取消自动上传
const beforeUpload: UploadProps['beforeUpload'] = async  (file) => {
  fileList.value = [file]
  try {
    const arrayBuffer = await file.arrayBuffer();
    fileData.value = new Uint8Array(arrayBuffer);
  } catch (error) {
    console.error('文件读取出错:', error);
  }
  return false
}

// 复制到剪切板
const copyToClipboard = (): void => {
  if (outPutStr.value) {
    navigator.clipboard.writeText(outPutStr.value).then(() => {
      globalProperties.$notification['success']({
        message: '已复制到剪切板'
      })
    }).catch(error => {
      console.error('无法复制文本: ', error);
    });
  }
}
</script>

<style scoped></style>