import ge from '..'
import { App, WebSocket } from 'uWebSockets.js'
import fs from 'fs'

const worldsDir = fs.readdirSync('./test/worlds')
let worlds = []
for (const world in worldsDir) {
  try {
    worlds.push(fs.readFileSync('./test/worlds/' + worldsDir[world]) + '')
  } catch {}
}
const config = fs.readFileSync('./test/config.json') + ''

const engine = new ge.ComputeEngine(new ge.EngineProps(config, worlds))

let lastId = 0
let clients = new Map<number, WebSocket<Client>>()
let clientsInput: Record<number, ge.Input> = {}
// const accessedKeys = ['left', 'right', 'up', 'down', 'shift']

interface Client {
  id: number
  input: ge.Input
  packages: object[]
  username: string
}

engine.onPlayerDeath((id) => {
  if (clients.get(id)) {
    clients.get(id)?.close()
  }
  return null
})

// for (let i = 0; i < 100; i++) {
//   engine.join(new JoinProps('EtherCD', i))
// }
// lastId = 100

App()
  .ws<Client>('/', {
    open: (ws) => {
      const data = ws.getUserData()
      data.id = lastId++
      data.input = new ge.Input()
      data.packages = []
      data.username = ''
      clientsInput[lastId - 1] = new ge.Input()
      // logger.info(`User connected ${data.id}`)
      clients.set(data.id, ws)
    },
    message: (ws, msg) => {
      const data = JSON.parse(Buffer.from(msg).toString())
      let client = ws.getUserData()!

      const keys = Object.keys(data)
      const input = clientsInput[client.id]

      for (const i of keys) {
        switch (i) {
          case 'message':
            engine.chatMessage(data.message, client.id)
            break
          case 'keyUp':
            switch (data[i]) {
              case 'down':
                input.setDown(false)
                break
              case 'left':
                input.setLeft(false)
                break
              case 'right':
                input.setRight(false)
                break
              case 'up':
                input.setUp(false)
                break
              case 'shift':
                input.setShift(false)
                break
            }
            break
          case 'keyDown':
            switch (data[i]) {
              case 'down':
                input.setDown(true)
                break
              case 'left':
                input.setLeft(true)
                break
              case 'right':
                input.setRight(true)
                break
              case 'up':
                input.setUp(true)
                break
              case 'shift':
                input.setShift(true)
                break
            }
            break
          case 'init':
            engine.join(new ge.JoinProps('EtherCD', client.id))
            break
          case 'mousePos':
            const pos = data[i]
            input.setMousePosX(pos[0])
            input.setMousePosY(pos[1])
            // mousePos(ws, data.mousePos!)
            break
          case 'mouseEnable':
            input.setMouseEnable(data[i])
            // mouseEnable(ws, data.mouseEnable!)
            break
          case 'ability':
            if (data.ability === 'first') input.setFirstAbility(true)
            if (data.ability === 'second') input.setSecondAbility(true)
            break
        }
      }
    },
    close: (ws: WebSocket<Client>) => {
      const client = ws.getUserData()
      if (client.id !== undefined) {
        clients.delete(client.id)
        console.info(`Client ${client.id} disconnected`)
        engine.leave(client.id)
      }
    },
  })
  .listen(7002, () => {
    console.info(`
   ___   ____                               
  / _ | / / /__  _____ _______ ___   _______
 / __ |/ / __/ |/ / -_) __(_-</ -_) / __(_-<
/_/ |_/_/\\__/|___/\\__/_/ /___/\\__/ /_/ /___/
                                            
`)

    console.log('Started at 7002')
  })

let timeout: NodeJS.Timeout

const tick = () => {
  clearTimeout(timeout)
  for (const index in clientsInput) {
    engine.input(Number(index), clientsInput[index])
    clientsInput[index].setFirstAbility(false)
  }
  const packages = engine.update() as Record<string, Buffer>

  for (const [id, client] of clients) {
    let pkg = packages[id]
    if (pkg != undefined) {
      if (pkg.length != 0) client.send(pkg, true)
    }
  }

  timeout = setTimeout(tick, 1000 / 60)
}

tick()
