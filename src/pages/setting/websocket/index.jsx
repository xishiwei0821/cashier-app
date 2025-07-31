import { useState } from 'react'
import { App, Flex, Tag, Input, Button } from 'antd'
import { invoke } from '@tauri-apps/api/core'

const WebsocketPage = () => {
  const [port, setPort] = useState(9898)
  const [status, setStatus] = useState(false)
  const [loading, setLoading] = useState(false)

  const { message } = App.useApp()

  const startWsServer = async () => {
    setLoading(true)

    try {
      const result = await invoke('start_ws_server', { port: port })

      console.log('启动结果', result)

      if (!result) {
        throw new Error('启动 ws 服务失败')
      }

      setStatus(true)
      message.success('启动成功')
    } catch (error) {
      console.log(error)
      message.error(typeof(error) == 'string' ? error : error.message || '未知错误')
    } finally {
      setLoading(false)
    }
  }

  const stopWsServer = async () => {
    setLoading(true)

    try {
      const result = await invoke('stop_ws_server')

      console.log('关闭结果', result)

      if (!result) {
        throw new Error('关闭失败')
      }

      setStatus(false)
      message.success('关闭成功')
    } catch (error) {
      console.log(error)
      message.error(typeof(error) == 'string' ? error : error.message || '未知错误')
    } finally {
      setLoading(false)
    }
  }

  return (
    <>
      <Flex gap={ '10px' }>
        <Input
          style={{ width: '180px' }}
          addonBefore="ws://127.0.0.1:"
          disabled={ status }
          value={ port }
          onChange={ (e) => setPort(Number(e.target.value)) }
        />
        <Button
          loading={ loading }
          variant='solid'
          color={ status ? 'danger' : 'primary' }
          onClick={ status ? stopWsServer : startWsServer }
        >{ status ? '关闭' : '启动' }</Button>
      </Flex>
      <p>状态: <Tag color={ status ? 'success' : 'error' }>{ status ? '已连接' : '未连接' }</Tag></p>
      { status && <p>当前连接地址: <Tag>ws://127.0.0.1:{ port }</Tag></p> }
    </>
  )
}

export default WebsocketPage
