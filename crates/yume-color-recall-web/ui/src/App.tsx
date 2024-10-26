import { useEffect, useState } from 'react'
import './App.css'
import game_wasm, {
  init_game, init_panic_hook, target_color_css
  , current_color_css, available_models, model_name, model_sliders, switch_model, update_slider,
  model_info_link,
  compute_score,
  color_acceptable
} from '../../pkg'
import './i18n'
import { Alert, Box, Button, Divider, FormControl, FormGroup, FormLabel, Link, MenuItem, Paper, Radio, RadioGroup, Select, Slider, Stack, Typography } from '@mui/material'
import { useTranslation } from 'react-i18next'

function ColorSampleBlock(props: { color: string, size: number, showText?: boolean }) {
  return (
    <Box
      sx={{
        backgroundColor: props.color,
        width: props.size,
        height: props.size,
        display: 'inline-block',
        align: 'center',
      }}
    >
      {props.showText ? <Typography variant="body1">{props.color}</Typography> : null}
    </Box>
  )
}

function NextBtn(props: { next?: () => void, reset: () => void }) {
  const { t } = useTranslation();
  return (
    <>
      {
        props.next ?
          <Button variant="contained" onClick={props.next} sx={{ margin: 1 }}>
            {t('btn_next')}
          </Button> : null}
      <Button variant="contained" onClick={props.reset} sx={{ margin: 1 }} color="warning">
        {t('btn_reset')}
      </Button>
    </>
  )
}


interface SliderInfo {
  name: string
  value: number
  min: number
  max: number
}

function Step4(props: { reset: () => void, lastModel: string, recall_ms: number }) {
  const { t } = useTranslation();
  return (
    <Box>
      <Typography variant="h4">{t('step_4_score')}</Typography>
      <Typography variant="h6">{t('step_4_instructions')}</Typography>
      <Divider sx={{ margin: 2 }} />

      <Typography variant="h6">{t('target_color')}</Typography>
      <ColorSampleBlock showText color={target_color_css()} size={100} />

      <Typography variant="h6">{t('your_color')}</Typography>
      <ColorSampleBlock showText color={current_color_css('srgb')} size={100} />

      <Divider sx={{ margin: 2 }} />

      <Typography variant="h6">{`CIEDE2000 Delta E, ${t('lower_is_better')}`}</Typography>
      <Typography variant="body1">{compute_score()}</Typography>

      <Divider sx={{ margin: 2 }} />

      <Typography variant="body1">{t('last_model') + t(props.lastModel)}</Typography>

      <Typography variant="body1">{t('time_taken_recall') + (props.recall_ms / 1000).toFixed(1)}s</Typography>

      <Divider sx={{ margin: 2 }} />

      <NextBtn reset={props.reset} />

    </Box>
  )
}

function Step3(props: {
  next: (
    lastModel: string,
    recall_ms: number
  ) => void, reset: () => void
}) {
  const { t } = useTranslation();

  const [beginRecall, setBeginRecall] = useState(0)
  const [availableModels, setAvailableModels] = useState<string[]>([])
  const [sliderMap, setSliderMap] = useState(new Map<string, SliderInfo[]>())
  const [updateCount, setUpdateCount] = useState(0)
  const [currentModel, setCurrentModel] = useState("")
  const [transferArray, _] = useState(new Float32Array([0.0, 0.0, 0.0, 0.0]))

  if (availableModels.length === 0) {
    setBeginRecall(+new Date())
    const ms = available_models()
    setAvailableModels(ms)
    switch_model('srgb')
    setUpdateCount(updateCount + 1)
    ms.forEach((model) => {
      const sliders = model_sliders(model)
      if (sliders) {
        setSliderMap((prev) => {
          prev.set(model, sliders.map((s) => {
            return {
              name: s.name,
              value: s.value,
              min: s.min,
              max: s.max
            }
          }))
          return new Map(prev)
        })
      } else {
        console.error(`No sliders for model ${model}`)
      }
    })
  }

  const unacceptable_reason = color_acceptable()

  return (
    <Box>
      <Typography variant="h4">{t('step_3_recall')}</Typography>
      <Typography variant="h6">{t('step_3_instructions')}</Typography>
      <ColorSampleBlock color={current_color_css(currentModel)} size={100} />
      <Divider sx={{ margin: 2 }} />
      <FormControl>
        <FormLabel>{t('colorspace')}</FormLabel>

        <RadioGroup row>
          {availableModels.map((model) => {
            return (
              <FormGroup key={model}>
                <Radio
                  value={model}
                  onChange={() => {
                    switch_model(currentModel)
                    setCurrentModel(model)
                    availableModels.forEach((m) => {
                      const newSliders = model_sliders(m)
                      if (newSliders) {
                        newSliders.forEach((s, i) => {
                          transferArray[i] = s.value
                        })
                        sliderMap.set(m, newSliders.map((s) => {
                          return {
                            name: s.name,
                            value: s.value,
                            min: s.min,
                            max: s.max
                          }
                        }))
                      }
                    })
                  }}
                />
                <Typography variant="body1">{model_name(model)}</Typography>
                <Link href={model_info_link(model)} target="_blank" rel="noreferrer"> Info </Link>
              </FormGroup>
            )
          })}
        </RadioGroup>
      </FormControl>
      <Box>
        {availableModels.map((model) => {
          if (model !== currentModel) {
            return null
          }
          const sliders = sliderMap.get(model)
          return sliders ? (
            <Box key={model} sx={{ display: 'inline-block', margin: 1 }}>
              <Typography variant="h6">{model_name(model)}</Typography>
              <Stack spacing={2}>
                {sliders.map((slider, j) => {
                  return (
                    <Box key={`${model}-${slider.name}-${j}`}>
                      <Typography variant="body1">{slider.name}</Typography>
                      <Stack spacing={2} sx={{ width: '20rem' }}>
                        <Typography variant="body2">{slider.value}</Typography>
                        <Slider
                          value={slider.value}
                          onChange={(_, v) => {
                            const new_value = v as number;
                            sliders.find(s => s.name === slider.name)!.value = new_value;
                            sliderMap.set(model, sliders)
                            sliders.forEach((s, i) => {
                              transferArray[i] = s.value
                            });
                            update_slider(model, transferArray)
                            setUpdateCount(updateCount + 1)
                          }}
                          min={slider.min}
                          max={slider.max}
                          step={(slider.max - slider.min) / 128}
                        />
                      </Stack>
                    </Box>
                  )
                })}
              </Stack>
              <Divider sx={{ margin: 2 }} />
            </Box>
          ) : null
        })}
        {
          unacceptable_reason ? (
            <Box>
              <Alert severity='warning'>
                {t('unacceptable_warning') + (t('unacceptable') as any)(unacceptable_reason)}
              </Alert>
            </Box>
          ) : null
        }
      </Box>

      <Divider sx={{ margin: 2 }} />
      <NextBtn next={() => props.next(model_name(currentModel), +new Date() - beginRecall)} reset={props.reset} />
    </Box>
  )

}

function Step2(props: { next: () => void, reset: () => void }) {
  const { t } = useTranslation();
  const delay = 8000
  const [remaining, setRemaining] = useState(delay)
  const [begin, _] = useState(+new Date())

  useEffect(() => {
    const interval = setInterval(() => {
      const elapsed = +new Date() - begin;
      setRemaining(delay - elapsed);
      if (elapsed >= delay) {
        props.next();
      }
    }, 100)


    return () => clearInterval(interval)
  }, [begin])

  return (
    <Box>
      <Typography variant="h4">{t('step_2_delay')}</Typography>
      <Typography variant="h6">{(remaining / 1000).toFixed(1)}s</Typography>
    </Box>
  )
}

function Step1(props: { next: () => void, reset: () => void }) {
  const { t } = useTranslation();
  return (
    <Box>
      <Typography variant="h4">{t('step_1_memorization')}</Typography>
      <Typography variant="h6">{t('step_1_instructions')}</Typography>
      <ColorSampleBlock color={target_color_css()} size={100} />
      <Divider sx={{ margin: 2 }} />
      <NextBtn next={props.next} reset={props.reset} />
    </Box>
  )
}

function LanguagePicker() {
  const { i18n } = useTranslation();
  return (
    <Box>
      <Select defaultValue={i18n.language} onChange={(e) => i18n.changeLanguage(e.target.value as string)}>
        <MenuItem value="en">English</MenuItem>
        <MenuItem value="zh">中文</MenuItem>
        <MenuItem value="ja">日本語</MenuItem>
      </Select>
    </Box>
  )
}

function Game() {
  const { t } = useTranslation();
  const [step, setStep] = useState(1)
  const [updateCount, setUpdateCount] = useState(0)
  const [recallTime, setRecallTime] = useState(0)
  const [lastModel, setLastModel] = useState("")

  const doReset = () => {
    init_game()
    setStep(1)
    setUpdateCount(updateCount + 1)
  }

  return (
    <Box>
      <Typography variant="h2">{t('title')}</Typography>
      <Typography variant="h6"><Link href="https://github.com/eternal-flame-AD/color-recall" target="_blank" rel="noreferrer">GitHub/README</Link></Typography>
      <LanguagePicker />
      <Divider sx={{ margin: 2 }} />
      {
        (step === 1) ?
          (<Step1 next={() => setStep(2)} reset={doReset} />)
          : null
      }
      {
        (step === 2) ?
          (<Step2 next={() => setStep(3)} reset={doReset} />)
          : null
      }
      {
        (step === 3) ?
          (<Step3 next={(model, recall_ms) => {
            setLastModel(model)
            setRecallTime(recall_ms)
            setStep(4)
          }} reset={doReset} />)
          : null
      }
      {
        (step === 4) ?
          (<Step4 reset={doReset} lastModel={lastModel} recall_ms={recallTime} />)
          : null
      }
    </Box>
  )
}

function App() {
  const [mounted, setMounted] = useState(false)
  const [ready, setReady] = useState(false)

  if (!mounted) {
    setMounted(true)
    game_wasm().then(() => {
      init_panic_hook()
      init_game()
      setReady(true)
    })
  }

  return <Paper sx={{ padding: 2 }}>
    {ready ? <Game /> : <p>Loading...</p>}
  </Paper>

}

export default App
