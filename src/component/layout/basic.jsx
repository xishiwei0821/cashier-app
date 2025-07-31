import { useState } from 'react'
import { Link, Outlet, useLocation } from 'react-router-dom'
import { Layout, Menu, Breadcrumb, Button, theme } from 'antd'
import LogoImg from '@/assets/react.svg'
import {
  HomeOutlined,
  MenuOutlined,
  MenuUnfoldOutlined,
  MenuFoldOutlined,
} from '@ant-design/icons'
import { menuRouters } from '@/config/routers'
import { userMenuRouter } from '@/hooks'

import './basic.css'

const { Header, Content, Footer, Sider } = Layout

const getItems = (menus) => {
  let items = []
  menus.map((route) => {
    let item = {
      key: route.path,
      label: route.children && route.children.length > 0 ? route.title : <Link to={ route.path } style={{ color: '#fff' }}>{ route.title }</Link>,
      icon: route.icon ? route.icon : <MenuOutlined />,
      router: route.path,
    }
    if (route.children && route.children.length > 0) {
      item.children = getItems(route.children)
    }

    items.push(item)
  })

  return items
}

const items = getItems(menuRouters)

const BasicLayout = () => {
  const location = useLocation()
  const menuPaths = userMenuRouter(menuRouters)
  const [collapsed, setCollapsed] = useState(false)
  const { token: { colorBgContainer, borderRADIUSLG } } = theme.useToken()

  const breadcrumb = [
    { title: <Link to="/"><HomeOutlined /> 首页</Link> },
    ...menuPaths.map((route, index) => {
      let content = <>{ route.icon ? route.icon : <MenuOutlined /> } { route.title }</>

      return {
        title: index === menuPaths.length - 1 ? <Link to={ route.path }>{ content }</Link> : content
      }
    })
  ]

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Sider trigger={ null } collapsible collapsed={ collapsed }>
        <div className="logo-vertical">
          <img src={ LogoImg } alt="" />
        </div>
        <Menu
          theme="dark"
          mode='inline'
          selectedKeys={[ location.pathname ]}
          items={ items }
        />
      </Sider>
      <Layout>
        <Header style={{ padding: 0, background: colorBgContainer }}>
          <Button
            type="text"
            style={{ fontSize: '16px', width: 64, height: 64 }}
            icon={ collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined /> }
            onClick={ () => setCollapsed(!collapsed) }
          />
        </Header>
        <Content style={{ margin: '0 16px' }}>
          <Breadcrumb style={{ margin: '16px 0' }} items={ breadcrumb } />
          <div style={{ padding: 24, minHeight: 200, background: colorBgContainer, borderRadius: borderRADIUSLG }}>
            <Outlet />
          </div>
        </Content>
        <Footer style={{ textAlign: 'center' }}>
          <p>Rust + Tauri + Vite + React + Antd Design</p>
          <p>©{ new Date().getFullYear() }</p>
        </Footer>
      </Layout>
    </Layout>
  )
}

export default BasicLayout
