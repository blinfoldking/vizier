import { useState, useEffect } from 'react'
import { useParams } from 'react-router'
import { FaBrain } from 'react-icons/fa6'
import { getAgentCore, updateAgentCore } from '../services/vizier'
import { useToastStore } from '../hooks/toastStore'
import MarkdownEditor from '../components/MarkdownEditor'

function getErrorMessage(err: unknown): string {
  if (err && typeof err === 'object' && 'response' in err) {
    const resp = (err as { response?: { data?: { message?: string } } }).response
    return resp?.data?.message || 'An error occurred'
  }
  return 'An error occurred'
}

export default function AgentCore() {
  const { agentId } = useParams()
  const addToast = useToastStore((s) => s.addToast)

  const [content, setContent] = useState('')
  const [original, setOriginal] = useState('')
  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)

  useEffect(() => {
    if (!agentId) return
    const load = async () => {
      setLoading(true)
      try {
        const res = await getAgentCore(agentId)
        const data = res.data?.content || ''
        setContent(data)
        setOriginal(data)
      } catch (err: unknown) {
        console.error('Failed to load CORE:', err)
        addToast('error', 'Failed to load CORE', getErrorMessage(err))
        setContent('')
        setOriginal('')
      } finally {
        setLoading(false)
      }
    }
    load()
  }, [agentId])

  const handleSave = async () => {
    if (!agentId) return
    setSaving(true)
    try {
      await updateAgentCore(agentId, content)
      setOriginal(content)
      addToast('success', 'CORE saved')
    } catch (err: unknown) {
      addToast('error', 'Failed to save CORE', getErrorMessage(err))
    } finally {
      setSaving(false)
    }
  }

  const handleReset = () => {
    setContent(original)
  }

  const hasChanges = content !== original

  return (
    <>
      <div className="main-header">
        <h3 style={{ margin: 0 }}>Core</h3>
        {hasChanges && (
          <div style={{ display: 'flex', gap: '0.5rem', alignItems: 'center' }}>
            <span style={{ fontSize: '12px', color: 'var(--text-tertiary)' }}>
              Unsaved changes
            </span>
            <button
              className="btn btn-ghost"
              onClick={handleReset}
              disabled={saving}
            >
              Reset
            </button>
            <button
              className="btn btn-primary"
              onClick={handleSave}
              disabled={saving}
            >
              {saving ? 'Saving...' : 'Save'}
            </button>
          </div>
        )}
      </div>

      <div className="main-body" style={{ padding: '1.5rem' }}>
        <p
          style={{
            color: 'var(--text-secondary)',
            fontSize: '14px',
            marginBottom: '1rem',
            maxWidth: '600px',
          }}
        >
          Persistent memory and identity for the agent. This document is included
          in the agent's system prompt and can be updated by the agent itself using
          the <code>WRITE_CORE</code> tool.
        </p>

        {loading ? (
          <p style={{ color: 'var(--text-tertiary)' }}>Loading CORE...</p>
        ) : (
          <div style={{ height: 'calc(100vh - 200px)' }}>
            <MarkdownEditor
              value={content}
              onChange={setContent}
              placeholder="Enter CORE content..."
              className="document-mdx-editor"
            />
          </div>
        )}
      </div>
    </>
  )
}
