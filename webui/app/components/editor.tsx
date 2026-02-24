import {
  MDXEditor,
  headingsPlugin,
  markdownShortcutPlugin,
  listsPlugin,
  quotePlugin,
  thematicBreakPlugin,
} from '@mdxeditor/editor'
import { useRef, useState } from 'react'
import { FaPaperPlane } from 'react-icons/fa'

const Editor = (props: { onSubmit: (value: string) => void }) => {
  let ref: any = useRef(null)
  const [value, setValue] = useState('')
  const [onFocus, setOnFocus] = useState(false)

  const submit = (value: string) => {
    console.log({ value })
    props.onSubmit(value)
    setValue('')
    ref?.current?.setMarkdown('')
  }

  return (
    <div className="w-full">
      <div
        className="max-h-[25vh] h-full w-full bg-white rounded-4xl pl-5 pr-5  shadow-md flex justify-center items-center"
        onKeyDown={(event) => {
          if (!event.shiftKey && event.key === 'Enter') {
            if (!value.trim()) {
              return
            }

            // Prevent the default soft-break behavior
            event.preventDefault()
            submit(value)
          }
        }}
        onFocus={() => {
          setOnFocus(true)
        }}
      >
        <div className="w-full max-h-[25vh] overflow-y-scroll no-scrollbar">
          <MDXEditor
            ref={ref}
            className="max-h-[25vh] prose"
            markdown={value}
            plugins={[
              headingsPlugin(),
              listsPlugin(),
              quotePlugin(),
              thematicBreakPlugin(),
              markdownShortcutPlugin(),
            ]}
            contentEditableClassName="editor-content"
            spellCheck={false}
            placeholder="Type Something!"
            onChange={setValue}
            onBlur={() => setOnFocus(false)}
          />
        </div>
        <div className="h-full max-h-[25vh] flex pt-2.5 pb-2.5">
          <div
            className="active:inset-shadow-md hover:inset-shadow-xs h-10 w-10 flex justify-center items-center rounded-full text-gray-500 hover:text-black"
            onClick={() => submit(value)}
          >
            <FaPaperPlane />
          </div>
        </div>
      </div>
    </div>
  )
}

export default Editor
