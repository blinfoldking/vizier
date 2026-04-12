import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from 'recharts'

export type UsageMetric = 'total' | 'input' | 'output'

interface UsageBarChartProps {
  data: Array<{
    name: string
    total: number
    input: number
    output: number
  }>
  metric: UsageMetric
}

const metricLabels: Record<UsageMetric, string> = {
  total: 'Total Tokens',
  input: 'Input Tokens',
  output: 'Output Tokens',
}

const CustomTooltip = ({ active, payload, label }: any) => {
  if (active && payload && payload.length) {
    return (
      <div
        style={{
          background: 'var(--surface)',
          border: '1px solid var(--border)',
          borderRadius: '4px',
          padding: '8px 12px',
          fontSize: '12px',
        }}
      >
        <p style={{ fontWeight: 600, marginBottom: '4px' }}>{label}</p>
        <p style={{ color: 'var(--accent-primary)' }}>
          Total: {payload[0]?.value?.toLocaleString() || 0}
        </p>
        <p style={{ color: 'var(--text-secondary)' }}>
          Input: {payload[0]?.payload?.input?.toLocaleString() || 0}
        </p>
        <p style={{ color: 'var(--text-secondary)' }}>
          Output: {payload[0]?.payload?.output?.toLocaleString() || 0}
        </p>
      </div>
    )
  }
  return null
}

export function UsageBarChart({ data, metric }: UsageBarChartProps) {
  return (
    <div style={{ width: '100%', height: 300 }}>
      <ResponsiveContainer>
        <BarChart
          data={data}
          margin={{ top: 20, right: 30, left: 20, bottom: 5 }}
        >
          <CartesianGrid strokeDasharray="3 3" stroke="var(--border)" />
          <XAxis
            dataKey="name"
            tick={{ fill: 'var(--text-secondary)', fontSize: 11 }}
            axisLine={{ stroke: 'var(--border)' }}
          />
          <YAxis
            tick={{ fill: 'var(--text-secondary)', fontSize: 11 }}
            axisLine={{ stroke: 'var(--border)' }}
            tickFormatter={(value) => value.toLocaleString()}
          />
          <Tooltip content={<CustomTooltip />} />
          <Bar
            dataKey={metric}
            fill="var(--accent-primary)"
            radius={[4, 4, 0, 0]}
            maxBarSize={50}
          />
        </BarChart>
      </ResponsiveContainer>
    </div>
  )
}

interface ChannelTypeBarChartProps {
  data: Array<{
    name: string
    total: number
    input: number
    output: number
  }>
  metric: UsageMetric
}

export function ChannelTypeBarChart({ data, metric }: ChannelTypeBarChartProps) {
  return (
    <div style={{ width: '100%', height: 300 }}>
      <ResponsiveContainer>
        <BarChart
          data={data}
          layout="horizontal"
          margin={{ top: 20, right: 30, left: 0, bottom: 5 }}
        >
          <CartesianGrid strokeDasharray="3 3" stroke="var(--border)" />
          <XAxis
            dataKey="name"
            tick={{ fill: 'var(--text-secondary)', fontSize: 11 }}
            axisLine={{ stroke: 'var(--border)' }}
          />
          <YAxis
            tick={{ fill: 'var(--text-secondary)', fontSize: 11 }}
            axisLine={{ stroke: 'var(--border)' }}
            width={70}
          />
          <Tooltip content={<CustomTooltip />} />
          <Bar
            dataKey={metric}
            fill="var(--accent-primary)"
            radius={[0, 4, 4, 0]}
            maxBarSize={30}
          />
        </BarChart>
      </ResponsiveContainer>
    </div>
  )
}
