import { useState } from 'react'
import { App, Flex, Input, Select, Button, Tag } from 'antd'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

const defaultPortList = [
  "COM1", "COM2"
]

const SerialPage = () => {
  const [portList, setPortList] = useState(defaultPortList)
  const [port, setPort] = useState('COM1')
  const [baudRate, setBaudRate] = useState(9600)
  const [status, setStatus] = useState(false)
  const [loading, setLoading] = useState(false)
  const [weight, setWeight] = useState("0.000")

  const { message } = App.useApp()

  const refreshPortList = async () => {
    try {
      const result = await invoke('read_serial_port')
      setPortList(result)
      setPort('')
    } catch (error) {
      console.log(error)
      message.error(typeof(error) == 'string' ? error : error.message || '未知错误')
    }
  }

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

  listen('serial_data', (event) => {
    const receivedData = event.payload
    console.log('接收到数据', receivedData)
    setWeight(receivedData)
  })

  return (
    <>
      <Flex gap={ '10px' }>
        <Select
          style={{ width: '160px' }}
          value={ port }
          disabled={ status }
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
        <Button onClick={ refreshPortList }>刷新串口队列</Button>
      </Flex>
      <p>状态: <Tag color={ status ? 'success' : 'error' }>{ status ? '已连接' : '未连接' }</Tag></p>
      <p>当前读数: { weight }</p>
    </>
  )
}

export default SerialPage
