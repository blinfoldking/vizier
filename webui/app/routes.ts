import {
  type RouteConfig,
  index,
  layout,
  route,
} from '@react-router/dev/routes'

export default [
  layout('layout.tsx', [
    index('routes/home.tsx'),
    route('agents/:agent_id', 'routes/chats.tsx'),
  ]),
] satisfies RouteConfig
