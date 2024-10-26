import { useEffect, useState } from 'react'
import './App.css'
import game_wasm, {
  init_game, init_panic_hook, target_color_css
  , current_color_css, available_models, model_name, model_sliders, switch_model, update_slider,
  model_info_link,
  compute_score
} from '../../pkg'
import { Box, Button, Divider, FormControl, FormGroup, FormLabel, Link, Paper, Radio, RadioGroup, Slider, Stack, Typography } from '@mui/material'

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

function NextBtn(props: { next: () => void }) {
  return (
    <Button variant="contained" onClick={props.next}>
      Next
    </Button>
  )
}

interface SliderInfo {
  name: string
  value: number
  min: number
  max: number
}

function Step4(props: { reset: () => void }) {
  return (
    <Box>
      <Typography variant="h4">Step 4 - Result</Typography>
      <Divider sx={{ margin: 2 }} />

      <Typography variant="h6">Target Color</Typography>
      <ColorSampleBlock showText color={target_color_css()} size={100} />

      <Typography variant="h6">Your Color</Typography>
      <ColorSampleBlock showText color={current_color_css('srgb')} size={100} />

      <Divider sx={{ margin: 2 }} />

      <Typography variant="h6">Score (CIEDE2000 Delta E, lower is better)</Typography>
      <Typography variant="body1">{compute_score()}</Typography>

      <Divider sx={{ margin: 2 }} />

      <NextBtn next={props.reset} />

    </Box>
  )
}

function Step3(props: { next: () => void }) {
  const [availableModels, setAvailableModels] = useState<string[]>([])
  const [sliderMap, setSliderMap] = useState(new Map<string, SliderInfo[]>())
  const [updateCount, setUpdateCount] = useState(0)
  const [currentModel, setCurrentModel] = useState("")
  const [transferArray, _] = useState(new Float32Array([0.0, 0.0, 0.0, 0.0]))

  if (availableModels.length === 0) {
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

  return (
    <Box>
      <Typography variant="h4">Step 3 - Selection</Typography>
      <Typography variant="h6">Reconstruct the color as close as possible :)</Typography>
      <ColorSampleBlock color={current_color_css(currentModel)} size={100} />
      <Divider sx={{ margin: 2 }} />
      <FormControl>
        <FormLabel>ColorSpace</FormLabel>

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
      </Box>

      <Divider sx={{ margin: 2 }} />

      <NextBtn next={props.next} />
    </Box>
  )

}

function Step2(props: { next: () => void }) {
  const delay = 5000
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
      <Typography variant="h4">Step 2 - Delay</Typography>
      <Typography variant="h6">Remaining: {(remaining / 1000).toFixed(1)}s</Typography>
    </Box>
  )
}

function Step1(props: { next: () => void }) {
  return (
    <Box>
      <Typography variant="h4">Step 1 - Memorization</Typography>
      <Typography variant="h6">Memorize the color below, take as much time as you need.</Typography>
      <ColorSampleBlock color={target_color_css()} size={100} />
      <Divider sx={{ margin: 2 }} />
      <NextBtn next={props.next} />
    </Box>
  )
}

function Game() {
  const [step, setStep] = useState(1)

  return (
    <div>
      <h1>Yumechi's Color Recall Test</h1>

      {
        (step === 1) ?
          (<Step1 next={() => setStep(2)} />)
          : null
      }
      {
        (step === 2) ?
          (<Step2 next={() => setStep(3)} />)
          : null
      }
      {
        (step === 3) ?
          (<Step3 next={() => setStep(4)} />)
          : null
      }
      {
        (step === 4) ?
          (<Step4 reset={() => { init_game(); setStep(1) }} />)
          : null
      }
    </div>
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
