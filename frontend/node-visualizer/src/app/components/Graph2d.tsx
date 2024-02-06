// components/Graph2d.tsx
import React, { useCallback, useRef } from "react";
import ForceGraph3D, { NodeObject } from "react-force-graph-2d";
// Throttled values
import { useResizeDetector } from 'react-resize-detector';
interface Graph2dProps {
  graphData: { nodes: GraphNode[]; links: GraphLink[] };
  nodeCanvasObject: (
    node: NodeObject<NodeObject<GraphNode>>,
    ctx: CanvasRenderingContext2D,
    globalScale: number
  ) => void;
  nodeAutoColorBy: string;
  linkDirectionalParticles: number;
}

export interface GraphNode {
  id: number;
  name: string;
  node_type: string;
}

export interface GraphLink {
  source: number;
  target: number;
}

const Graph2d: React.FC<Graph2dProps> = ({
  graphData,
  nodeAutoColorBy,
  nodeCanvasObject,
  linkDirectionalParticles,
}) => {
  const { width, height, ref } = useResizeDetector({
    handleHeight: false,
    refreshMode: 'debounce',
    refreshRate: 200,
  });
  return (
    <div
    ref={ref}
    className="h-full w-full bg-charcoal p-4 rounded-lg shadow-lg"
    >
      <ForceGraph3D
        width={width}
        height={height}
        nodeAutoColorBy={nodeAutoColorBy}
        nodeCanvasObject={nodeCanvasObject}
        linkDirectionalParticles={linkDirectionalParticles}
        graphData={graphData}
      />
     
    </div>
  );
};

export default Graph2d;
