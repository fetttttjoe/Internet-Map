import React, { useEffect, useRef, useState } from 'react'
import ForceGraph2D, { NodeObject } from 'react-force-graph-2d'

interface Graph2dProps {
  graphData: { nodes: GraphNode[]; links: GraphLink[] }
  nodeCanvasObject?: (
    node: NodeObject<NodeObject<GraphNode>>,
    ctx: CanvasRenderingContext2D,
    globalScale: number
  ) => void
  nodeAutoColorBy: string
  linkDirectionalParticles: number
}

export interface GraphNode {
  id: number
  name: string
  node_type: string
}

export interface GraphLink {
  source: number
  target: number
}

const Graph2d: React.FC<Graph2dProps> = ({
  graphData,
  nodeAutoColorBy,
  nodeCanvasObject,
  linkDirectionalParticles,
}) => {
  const containerRef = useRef<HTMLDivElement>(null)
  // THIS COMPONENT IS UGLY AND NEEDS TO BE REFACTORED, plugin might needs to be switched
  const graphRef = useRef<any>(null)

  const [displayWidth, setDisplayWidth] = useState(window.innerWidth)
  const [displayHeight, setDisplayHeight] = useState(window.innerHeight)

  // Initialize the graph on component mount
  useEffect(() => {
    const container = containerRef.current
    const graphInstance = graphRef.current
    if (container && graphInstance) {
      const rect = container.getBoundingClientRect()
      setDisplayWidth(rect.width)
      setDisplayHeight(rect.height)
    }

    // Set up event listener for window resize
    window.addEventListener('resize', resizeGraph)
    // Clean up event listener on component unmount
    return () => {
      window.removeEventListener('resize', resizeGraph)
    }
  }, [])

  const resizeGraph = () => {
    const container = containerRef.current
    if (container) {
      const rect = container.getBoundingClientRect()
      const width = rect.width
      const height = rect.height
      const graphInstance = graphRef.current
      if (graphInstance) {
        setDisplayWidth(width)
        setDisplayHeight(height)
        graphInstance.zoomToFit(100)
      }
    }
  }
  return (
    <div
      id="graph-container"
      className="box-shadow: 0px 19px 113px 0px rgba(0,0,0,0.3) inset, 0px
      15px 12px 0px rgba(0,0,0,0.22); m-4"
      ref={containerRef}
    >
      <ForceGraph2D
        ref={graphRef}
        width={displayWidth}
        height={displayHeight}
        nodeAutoColorBy={nodeAutoColorBy}
        nodeCanvasObject={nodeCanvasObject}
        linkDirectionalParticles={linkDirectionalParticles}
        graphData={graphData}
      />
    </div>
  )
}

export default Graph2d
