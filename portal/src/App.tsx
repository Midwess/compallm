import { createBrowserRouter, Link, Outlet, useNavigation } from 'react-router'
import { authController } from './auth/controller'
import { guestOnly, requireCurrentUser, organizationIndex } from './auth/route-access'

function Layout() {
  const navigation = useNavigation()
  return (
    <>
      {navigation.state !== 'idle' ? <p role="status">Loading page…</p> : null}
      <Outlet />
    </>
  )
}

function RouteError() {
  return (
    <main>
      <h1>Unable to load this page</h1>
      <p>Please reload to try again.</p>
      <button type="button" onClick={() => window.location.reload()}>
        Reload
      </button>
    </main>
  )
}

function InitialLoading() {
  return <p role="status">Loading page…</p>
}

function NotFound() {
  return (
    <main>
      <h1>Page not found</h1>
      <Link to="/">Create an account</Link>
    </main>
  )
}

export const router = createBrowserRouter([
  {
    Component: Layout,
    ErrorBoundary: RouteError,
    HydrateFallback: InitialLoading,
    children: [
      { index: true, loader: guestOnly, lazy: () => import('./routes/signup') },
      { path: 'login', loader: guestOnly, lazy: () => import('./routes/login') },
      {
        path: ':organizationName',
        loader: requireCurrentUser,
        lazy: () => import('./routes/organization'),
        children: [
          { index: true, loader: organizationIndex, Component: () => null },
          { path: 'chat', lazy: () => import('./routes/chat') },
        ],
      },
      { path: '*', Component: NotFound },
    ],
  },
])

authController.navigate = (href) => {
  void router.navigate(href, { replace: href === '/login' })
}
