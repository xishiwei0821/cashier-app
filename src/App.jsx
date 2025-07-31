import { ConfigProvider, App as AntdApp } from 'antd'
import zhCN from 'antd/lib/locale/zh_CN'
import { RouterProvider } from 'react-router-dom'
import routers from '@/config/routers'

import "./App.css";

const theme = {

}

const App = () => {
  return (
    <ConfigProvider locale={ zhCN } theme={ theme }>
      <AntdApp>
        <RouterProvider router={ routers } />
      </AntdApp>
    </ConfigProvider>
  );
}

export default App;
