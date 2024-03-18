<template>
  <div style="background-color: #ececec; padding: 10px; height: 100%;">
    <a-row :gutter="8" style="height: 100%;">
      <a-col :span="12" style="height: 100%;">
        <a-card :bordered="false" style="height: 100%;">
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
import forge from 'node-forge'
// 获取全局配置属性
const instance = getCurrentInstance()
const globalProperties = instance!.appContext.config.globalProperties
// 已上传文件列表
const fileList = ref<UploadProps['fileList']>([])

const fileData = ref(new Uint8Array());
// 输出字符串
const outPutStr = computed(() => {
  if (fileData.value.length > 0){
    const base64Cert = Buffer.from(fileData.value).toString('base64')
    // 将BASE64编码的证书转换为PEM格式
    const pemCert = `-----BEGIN CERTIFICATE-----\n${base64Cert}\n-----END CERTIFICATE-----`;
    // 使用forge来解析PEM格式的证书
    const cert = forge.pki.certificateFromPem(pemCert);
    console.log(cert)
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
