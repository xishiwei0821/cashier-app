import { lazy } from 'react'
import { createHashRouter as createRouter } from 'react-router-dom'
import {
  HomeOutlined,
  SettingOutlined,
  PrinterOutlined,
  UsbOutlined,
  OneToOneOutlined,
} from '@ant-design/icons'

const menuRouters = [
  { path: '/', title: '首页', icon: <HomeOutlined />, Component: lazy(() => import('@/pages/home/index')) },
  { path: '/setting', title: '设置', icon: <SettingOutlined />, Component: lazy(() => import('@/component/empty')), children: [
    { path: '/setting/printer', title: '打印机设置', icon: <PrinterOutlined />, Component: lazy(() => import('@/pages/setting/printer/index')) },
    { path: '/setting/serial', title: '电子秤设置', icon: <UsbOutlined />, Component: lazy(() => import('@/pages/setting/serial/index')) },
    { path: '/setting/websocket', title: 'websocket设置', icon: <OneToOneOutlined />, Component: lazy(() => import('@/pages/setting/websocket/index')) },
  ] },
]

const routers = createRouter([
  { path: '/', Component: lazy(() => import('@/component/layout/basic')), children: menuRouters},
  { path: '/login', title: '登录', Component: lazy(() => import('@/pages/auth/login')) },
  { path: '/register', title: '注册', Component: lazy(() => import('@/pages/auth/register')) },
  { path: '/403', title: '无权限', Component: lazy(() => import('@/component/exception/403')) },
  { path: '/500', title: '服务器错误', Component: lazy(() => import('@/component/exception/500')) },
  { path: '*', title: '页面不存在', Component: lazy(() => import('@/component/exception/404')) },
])

export { menuRouters }
export default routers
