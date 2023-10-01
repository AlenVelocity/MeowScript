import React, { useEffect, useRef, useState } from 'react';
import Editor from 'react-simple-code-editor';
import Prism, { highlight } from 'prismjs';
import classNames from 'classnames';
import useBreakpoint from 'use-breakpoint';

import { MeowScriptWorker } from '../lib/MeowScriptWorker';
import { createMeowScriptWorker } from '../lib/create_meowscript_worker';
import { examples } from '../lib/example';
import { meowScriptSyntax } from '../lib/syntax';
import CodeDropdown from './CodeDropdown';
import 'prismjs/themes/prism-tomorrow.css'

const BREAKPOINTS = { mobile: 0, tablet: 768, desktop: 1280 }

const defaultText =
  '<h1 class="italic text-gray-300 border-b-2 border-gray-50 border-opacity-10 mb-1 pb-1"><span class="font-bold text-green-500">Let\'s paw-ceed with some purr-gramming!</h1>'

const hightlightWithLineNumbers = (input: string) =>
  highlight(input, meowScriptSyntax, 'meowScript')
    .split('\n')
    .map((line, i) => `<span class='editorLineNumber'>${i + 1}</span>${line}`)
    .join('\n')

const Code = () => {
  const [code, setCode] = useState<string>(examples.yarn)
  const [terminalText, setTerminalText] = useState<string>(defaultText)
  const { breakpoint } = useBreakpoint(BREAKPOINTS, 'mobile')

  const [loaded, setLoaded] = useState<boolean>(false)
  const [running, setRunning] = useState<boolean>(false)
  const [isUpsideDown, setIsUpsideDown] = useState<boolean>(false)

  const meowScriptRef = useRef<MeowScriptWorker>()
  const rightsideUpCached = useRef<string>('')

  const switchExample = (code: string) => {
    rightsideUpCached.current = code
    setIsUpsideDown(false)
    setCode(code)
  }

  useEffect(() => {
    const run = async () => {
      const [meowScript, worker] = await createMeowScriptWorker()
      worker.onmessage = e => {
        switch (e.data.type) {
          case 'stderr': {
            setTerminalText(
              text =>
                text +
                `<span class="text-red-500">${e.data.data}</span>` +
                '<br>'
            )
            break
          }
          case 'stdout': {
            setTerminalText(text => text + e.data.data + '<br>')
            break
          }
        }
      }
      meowScriptRef.current = meowScript
      setLoaded(true)
    }
    run()
    Prism.highlightAll()
  }, [])
  return (
    <div className="md:mt-24 bg-bg" id="code">
      <div className="flex flex-col justify-center items-center">
        <div className="p-4 lg:p-0 flex-col mx-auto pt-4">
          <div className="whitespace-nowrap flex justify-center items-center language-meowScript color-[#ccc]">
            <Editor
              style={{
                fontFamily:
                  "Consolas, Monaco, 'Andale Mono', 'Ubuntu Mono', monospace",
                overflowY: 'scroll',
                overflowX: 'hidden',
                wordBreak: 'keep-all',
                height:
                  breakpoint === 'mobile'
                    ? '90vh'
                    : breakpoint === 'tablet'
                    ? '75vh'
                    : '80vh',
                width: '100vw'
              }}
              textareaClassName="overflow-x-hidden whitespace-nowrap"
              textareaId="codeArea"
              className="flex-none whitespace-nowrap editor overflow-y-scroll overflow-x-scroll rounded-md text-[#ccc] bg-[#2d2d2d]"
              highlight={(code: string) => hightlightWithLineNumbers(code)}
              onValueChange={(v: string) => setCode(v || '')}
              value={code}
            />
          </div>
          <div className="flex flex-col justify-between">
            <div className="flex flex-row items-center justify-between mt-8 px-4">
              <div className="flex flex-row">
                <button
                  onClick={async () => {
                    if (running) return
                    setTerminalText(defaultText)
                    setRunning(true)
                    try {
                      await meowScriptRef.current?.run(code, isUpsideDown)
                    } catch (err) {
                      console.log(err)
                    }
                    setRunning(false)
                  }}
                  type="button"
                  className={classNames(
                    'items-center px-2.5 py-1.5 border border-transparent text-xs font-medium rounded shadow-sm text-white bg-orange-600 hover:bg-orange-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-orange-500',
                    !loaded || running ? 'cursor-not-allowed' : ''
                  )}>
                  Run
                </button>
                <button
                  onClick={() => setTerminalText(defaultText)}
                  type="button"
                  className="ml-4 inline-flex items-center px-2.5 py-1.5 border border-transparent text-xs font-medium rounded text-orange-700 bg-orange-100 hover:bg-orange-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-orange-500">
                  Clear
                </button>
              </div>
              <CodeDropdown setCode={switchExample} />
            </div>
            <div className="px-4 pt-2 mt-8 w-full rounded-md text-gray-50">
              <div
                className="bg-[#282C42] h-[18.75rem] overflow-auto px-4 pt-2 rounded-md"
                dangerouslySetInnerHTML={{ __html: terminalText }}></div>
            </div>
            <p className="text-gray-300 text-center my-4">
              Running with WebAssembly (Wasm) compiled from Rust ✨
            </p>
          </div>
        </div>
      </div>
    </div>
  )
}

export default Code
