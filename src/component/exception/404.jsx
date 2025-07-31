import { useLocation } from 'react-router-dom'

const NotFoundPage = () => {
  // 获取当前页面路由
  const location = useLocation()

  console.log(location.pathname)

  return (
    <>Ohhhhh 404 Not Found</>
  )
}

export default NotFoundPage
