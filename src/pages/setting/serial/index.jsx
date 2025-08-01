import { useState } from 'react'
import { App, Flex, Input, Select, Button, Tag } from 'antd'
import { invoke } from '@tauri-apps/api/core'

const portList = [
  "COM1", "COM2"
]

const SerialPage = () => {
  const [port, setPort] = useState('COM1')
  const [baudRate, setBaudRate] = useState(9600)
  const [status, setStatus] = useState(false)
  const [loading, setLoading] = useState(false)

  const { message } = App.useApp()

  const startSerialServer = async () => {
    setLoading(true)

    try {
      const result = await invoke('start_serial_server', { portName: port, baudRate: baudRate })

      console.log('启动结果', result)

      if (!result) {
        throw new Error('启动 串口 服务失败')
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

  const stopSerialServer = async () => {
    setLoading(true)

    try {
      const result = await invoke('stop_serial_server')

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
        <Select
          style={{ width: '160px' }}
          value={ port }
          options={portList.map(port => ({
            label: port,
            value: port,
          }))}
          onChange={ (e) => setPort(e) }
        />
        <Input
          style={{ width: '80px' }}
          disabled={ status }
          value={ baudRate }
          onChange={ (e) => setBaudRate(Number(e.target.value)) }
        />
        <Button
          loading={ loading }
          variant='solid'
          color={ status ? 'danger' : 'primary' }
          onClick={ status ? stopSerialServer : startSerialServer }
        >{ status ? '关闭' : '启动' }</Button>
      </Flex>
      <p>状态: <Tag color={ status ? 'success' : 'error' }>{ status ? '已连接' : '未连接' }</Tag></p>
    </>
  )
}

export default SerialPage
