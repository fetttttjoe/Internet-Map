// components/Graph2d.tsx
import React, { MutableRefObject, useCallback, useEffect, useRef } from 'react';
import ForceGraph3D, { ForceGraphMethods, LinkObject, NodeObject } from 'react-force-graph-2d';
interface Graph2dProps {
  graphData: { nodes: GraphNode[]; links: GraphLink[] };
  nodeCanvasObject: (node: NodeObject<NodeObject<GraphNode>>, ctx: CanvasRenderingContext2D, globalScale: number) => void;
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


const Graph2d: React.FC<Graph2dProps> = ({ graphData, nodeAutoColorBy, nodeCanvasObject, linkDirectionalParticles }) => {
 /**
 *  Make Responsive
 */
  return (
    <ForceGraph3D
    nodeAutoColorBy={nodeAutoColorBy}
    nodeCanvasObject={nodeCanvasObject}
    linkDirectionalParticles={linkDirectionalParticles}
    graphData={graphData}
    />
  );
};

export default Graph2d;
