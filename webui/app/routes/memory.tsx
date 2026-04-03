import { useEffect, useState } from 'react'
import { useParams } from 'react-router'
import { listMemories, getMemory, createMemory, updateMemory, deleteMemory } from '../services/vizier'
import { autoCorrectSlug, autoCorrectSlugStrict } from '../utils/slug'
import { FiPlus, FiTrash2 } from 'react-icons/fi'
import type { Memory, MemoryDetail } from '../interfaces/types'

type ModalMode = 'create' | 'edit' | 'view' | null

export default function MemoryManagement() {
  const { agentId } = useParams()
  const [memories, setMemories] = useState<Memory[]>([])
  const [selectedMemory, setSelectedMemory] = useState<MemoryDetail | null>(null)
  const [loading, setLoading] = useState(true)
  const [modalMode, setModalMode] = useState<ModalMode>(null)
  
  // Form state
  const [formTitle, setFormTitle] = useState('')
  const [formContent, setFormContent] = useState('')
  const [formSlug, setFormSlug] = useState('')
  const [submitting, setSubmitting] = useState(false)

  useEffect(() => {
    loadMemories()
  }, [agentId])

  const loadMemories = async () => {
    if (!agentId) return
    
    try {
      setLoading(true)
      const response = await listMemories(agentId)
      setMemories(response.data || [])
    } catch (error) {
      console.error('Failed to load memories:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleViewMemory = async (slug: string) => {
    if (!agentId) return
    
    try {
      const response = await getMemory(agentId, slug)
      setSelectedMemory(response.data)
      setModalMode('view')
    } catch (error) {
      console.error('Failed to load memory:', error)
    }
  }

  const handleEditMemory = (memory: MemoryDetail) => {
    setSelectedMemory(memory)
    setFormTitle(memory.title)
    setFormContent(memory.content)
    setFormSlug(memory.slug)
    setModalMode('edit')
  }

  const handleCreateMemory = () => {
    setFormTitle('')
    setFormContent('')
    setFormSlug('')
    setModalMode('create')
  }

  const handleSubmit = async () => {
    if (!agentId || !formTitle.trim() || !formContent.trim()) return
    
    setSubmitting(true)
    try {
      // Apply strict validation to finalize slug if provided
      const finalSlug = formSlug ? autoCorrectSlugStrict(formSlug) : undefined

      if (modalMode === 'create') {
        await createMemory(agentId, formTitle, formContent, finalSlug || undefined)
      } else if (modalMode === 'edit' && selectedMemory) {
        await updateMemory(agentId, selectedMemory.slug, formTitle, formContent)
      }
      
      await loadMemories()
      closeModal()
    } catch (error) {
      console.error('Failed to save memory:', error)
      alert('Failed to save memory')
    } finally {
      setSubmitting(false)
    }
  }

  const handleDeleteMemory = async (slug: string) => {
    if (!agentId) return
    if (!confirm('Are you sure you want to delete this memory?')) return
    
    try {
      await deleteMemory(agentId, slug)
      await loadMemories()
      closeModal()
    } catch (error) {
      console.error('Failed to delete memory:', error)
      alert('Failed to delete memory')
    }
  }

  const closeModal = () => {
    setModalMode(null)
    setSelectedMemory(null)
    setFormTitle('')
    setFormContent('')
    setFormSlug('')
  }

  if (loading) {
    return (
      <div className="main-body" style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
      }}>
        Loading memories...
      </div>
    )
  }

  return (
    <>
      {/* Header */}
      <div className="main-header">
        <div style={{ flex: 1 }}>
          <h3 style={{ margin: 0 }}>Memory Management</h3>
          <div style={{
            fontSize: '12px',
            color: 'var(--text-tertiary)',
            marginTop: '4px',
            fontFamily: 'var(--font-mono)',
          }}>
            Agent: {agentId}
          </div>
        </div>
        <button className="btn btn-primary" onClick={handleCreateMemory}>
          <FiPlus size={16} />
          <span>New Memory</span>
        </button>
      </div>

      {/* Memory List */}
      <div className="main-body">
        {memories.length === 0 ? (
          <div style={{
            textAlign: 'center',
            color: 'var(--text-tertiary)',
            padding: '3rem',
          }}>
            <p>No memories yet.</p>
            <button 
              className="btn btn-primary" 
              onClick={handleCreateMemory}
              style={{ marginTop: '1rem' }}
            >
              Create your first memory
            </button>
          </div>
        ) : (
          <div style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(auto-fill, minmax(300px, 1fr))',
            gap: '1rem',
          }}>
            {memories.map((memory) => (
              <div
                key={memory.slug}
                className="card"
                style={{
                  cursor: 'pointer',
                  transition: 'all 0.15s ease',
                }}
                onClick={() => handleViewMemory(memory.slug)}
              >
                <h4 style={{ marginBottom: '0.5rem' }}>{memory.title}</h4>
                <p style={{
                  fontSize: '12px',
                  color: 'var(--text-tertiary)',
                  marginBottom: '0.5rem',
                }}>
                  {memory.slug}
                </p>
                <p style={{
                  fontSize: '12px',
                  color: 'var(--text-tertiary)',
                }}>
                  {new Date(memory.timestamp).toLocaleString()}
                </p>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Modal */}
      {modalMode && (
        <>
          {/* Backdrop */}
          <div
            style={{
              position: 'fixed',
              top: 0,
              left: 0,
              right: 0,
              bottom: 0,
              background: 'rgba(0, 0, 0, 0.5)',
              zIndex: 1000,
            }}
            onClick={closeModal}
          />

          {/* Modal Content */}
          <div
            style={{
              position: 'fixed',
              top: '50%',
              left: '50%',
              transform: 'translate(-50%, -50%)',
              background: 'var(--background)',
              borderRadius: '8px',
              padding: '2rem',
              maxWidth: '700px',
              width: '90%',
              maxHeight: '80vh',
              overflow: 'auto',
              zIndex: 1001,
              border: '1px solid var(--border)',
            }}
          >
            {modalMode === 'view' && selectedMemory && (
              <>
                <div style={{
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'flex-start',
                  marginBottom: '1.5rem',
                }}>
                  <div>
                    <h2 style={{ marginBottom: '0.5rem' }}>{selectedMemory.title}</h2>
                    <p style={{
                      fontSize: '12px',
                      color: 'var(--text-tertiary)',
                    }}>
                      {selectedMemory.slug} • {new Date(selectedMemory.timestamp).toLocaleString()}
                    </p>
                  </div>
                  <button className="btn btn-ghost" onClick={closeModal}>✕</button>
                </div>
                
                <div className="prose" style={{
                  marginBottom: '1.5rem',
                  whiteSpace: 'pre-wrap',
                }}>
                  {selectedMemory.content}
                </div>

                <div style={{ display: 'flex', gap: '8px' }}>
                  <button
                    className="btn btn-secondary"
                    onClick={() => handleEditMemory(selectedMemory)}
                  >
                    Edit
                  </button>
                  <button
                    className="btn btn-ghost"
                    onClick={() => handleDeleteMemory(selectedMemory.slug)}
                    style={{ color: '#c00' }}
                  >
                    <FiTrash2 size={16} />
                    <span>Delete</span>
                  </button>
                </div>
              </>
            )}

            {(modalMode === 'create' || modalMode === 'edit') && (
              <>
                <div style={{
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'center',
                  marginBottom: '1.5rem',
                }}>
                  <h2>{modalMode === 'create' ? 'Create Memory' : 'Edit Memory'}</h2>
                  <button className="btn btn-ghost" onClick={closeModal}>✕</button>
                </div>

                <div style={{
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '1rem',
                }}>
                  {modalMode === 'create' && (
                    <div className="input-group">
                      <label htmlFor="slug">Slug (optional)</label>
                      <input
                        id="slug"
                        type="text"
                        value={formSlug}
                        onChange={(e) => setFormSlug(autoCorrectSlug(e.target.value))}
                        placeholder="auto-generated if empty"
                      />
                      {formSlug && (
                        <div style={{ fontSize: '12px', color: 'var(--text-tertiary)', marginTop: '4px' }}>
                          Slug: {formSlug}
                        </div>
                      )}
                    </div>
                  )}

                  <div className="input-group">
                    <label htmlFor="title">Title</label>
                    <input
                      id="title"
                      type="text"
                      value={formTitle}
                      onChange={(e) => setFormTitle(e.target.value)}
                      required
                      autoFocus
                    />
                  </div>

                  <div className="input-group">
                    <label htmlFor="content">Content</label>
                    <textarea
                      id="content"
                      value={formContent}
                      onChange={(e) => setFormContent(e.target.value)}
                      required
                      rows={10}
                      style={{ fontFamily: 'var(--font-mono)' }}
                    />
                  </div>

                  <div style={{ display: 'flex', gap: '8px', marginTop: '0.5rem' }}>
                    <button
                      className="btn btn-primary"
                      onClick={handleSubmit}
                      disabled={!formTitle.trim() || !formContent.trim() || submitting}
                    >
                      {submitting ? 'Saving...' : 'Save'}
                    </button>
                    <button
                      className="btn btn-secondary"
                      onClick={closeModal}
                      disabled={submitting}
                    >
                      Cancel
                    </button>
                  </div>
                </div>
              </>
            )}
          </div>
        </>
      )}
    </>
  )
}
