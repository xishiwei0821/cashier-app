import { useEffect, useState } from 'react'
import { useLocation } from 'react-router-dom'

const useMenuRouter = (menuRouters) => {
  const location = useLocation()
  const [ menuList, setMenuList ] = useState([])

  useEffect(() => {
    const findMenuPaths = (routers, path, parents = []) => {
      for (const route of routers) {
        const currentPath = route.path

        if (currentPath === path) return [ ...parents, route ]

        if (route.children && route.children.length > 0) {
          const found = findMenuPaths(route.children, path, [...parents, route])
          if (found) return found
        }
      }

      return null
    }
    
    const menus = findMenuPaths(menuRouters, location.pathname) || []
    setMenuList([ ...menus ])
  }, [location.pathname, menuRouters])

  return menuList
}

export default useMenuRouter
