# Engine-Architecture

## Application

- manage, update and process events for [stages](#stage)
- contains and update [systems](#system) for the engine (exp.: Rendering System, Entity-Component System, Windowing System etc.)

## Systems
communicates with App through [Events]()
- Rendering System
- Physic System
- Input System
- Node System (ECS)
- Window System
- Layer System
- Log System

## Stacks
- Entity-Component Stack
- Render-Target Stack
- Node Stack
- Window Stack
- Layer Stack
