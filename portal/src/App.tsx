import { createBrowserRouter, Link, Outlet, useNavigation } from 'react-router'
import { urlController } from './url/controller'
import { redirect } from 'react-router'
import { guestOnly, requireCurrentUser, organizationIndex, confirmationAccess } from './auth/route-access'

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
      { path: 'signup', loader: guestOnly, lazy: () => import('./routes/signup') },
      { path: 'login/confirmation', loader: confirmationAccess, lazy: () => import('./routes/login') },
      { path: 'signup/confirmation', loader: confirmationAccess, lazy: () => import('./routes/signup') },
      {
        path: ':organizationName',
        loader: requireCurrentUser,
        shouldRevalidate: ({ currentParams, nextParams }) => currentParams.organizationName !== nextParams.organizationName,
        lazy: () => import('./routes/organization'),
        children: [
          { index: true, loader: organizationIndex, Component: () => null },
          {
            path: 'settings',
            children: [
              { index: true, loader: () => redirect('general') },
              { path: ':page/:id?', lazy: () => import('./routes/settings') },
            ],
          },
          { path: 'chat', lazy: () => import('./routes/chat') },
          {
            path: 'providers/new',
            lazy: () => import('./routes/create-provider'),
            children: [
              {
                index: true,
                Component: () => null,
                loader: ({ request }) => {
                  const url = new URL(request.url)
                  return redirect(url.pathname.replace(/\/$/, '') + '/choose-provider' + url.search)
                },
              },
              { path: 'choose-provider', Component: () => null },
              { path: 'configure-provider', Component: () => null },
            ],
          },
        ],
      },
      { path: '*', Component: NotFound },
    ],
  },
])

urlController.connect(router)
