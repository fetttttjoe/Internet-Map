// components/TreeGraph.tsx
'use client';
import React, { useEffect, useRef, useState } from 'react';
import axios from 'axios';
import dynamic from 'next/dynamic';
import { GraphLink, GraphNode } from './Graph2d';
import { NodeObject } from 'react-force-graph-2d';

interface Tree {
  nodes: TreeNode[];
  counter: number;
}

interface TreeNode {
  id: number;
  name: string;
  node_type: string;
  children: TreeNode[] | null;
}

const NoSSRForceGraph2D = dynamic(() => import('./Graph2d'), { ssr: false });

interface TreeGraphProps {}
const TreeGraph: React.FC<TreeGraphProps> = () => {
  const graphContainerRef = useRef<HTMLDivElement>(null);
  const [tree, setTree] = useState<Tree>();
  useEffect(() => {
    async function fetchData() {
      try {
        const response = await axios.get<Tree>('http://127.0.0.1:3200/tree');
        setTree(response.data);
      } catch (error) {
        console.error('Error fetching tree data:', error);
      }
    }

    fetchData();
  }, []);


  const flattenTree = (tree: Tree) => {
    const flattenedNodes: GraphNode[] = [];
    const flattenedLinks: GraphLink[] = [];

    const flatten = (node: TreeNode | null, parentId?: number) => {
      if (node) {
        const newNode: GraphNode = {
          id: node.id,
          name: node.name,
          node_type: node.node_type,
        };
        flattenedNodes.push(newNode);

        if (parentId !== undefined) {
          flattenedLinks.push({ source: parentId, target: node.id });
        }

        if (node.children) {
          node.children.forEach((child) => flatten(child, node.id));
        }
      }
    };

    // Iterate through all root nodes
    tree?.nodes.forEach((rootNode) => {
      flatten(rootNode);
    });

    return { nodes: flattenedNodes, links: flattenedLinks };
  };

  const drawCircle = (
    ctx: CanvasRenderingContext2D,
    x: number,
    y: number,
    options: {
      fillColor?: string;
      strokeColor?: string;
      strokeWidth?: number;
      radius?: number;
    } = {}
  ) => {
    const { fillColor = 'rgba(66, 153, 225, 0.8)', strokeColor = 'rgba(255, 255, 255, 0.8)', strokeWidth = 2, radius = 10 } = options;
  
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, 2 * Math.PI);
    ctx.fillStyle = fillColor;
    ctx.strokeStyle = strokeColor;
    ctx.lineWidth = strokeWidth;
    ctx.fill();
    ctx.stroke();
    ctx.closePath();
  };
  const drawRoundedRectangle = (
    ctx: CanvasRenderingContext2D,
    x: number,
    y: number,
    width: number,
    height: number,
    borderRadius: number,
    backgroundColor: string,
    borderColor: string,
    borderWidth: number
  ) => {
    ctx.beginPath();
    ctx.fillStyle = backgroundColor;
    ctx.strokeStyle = borderColor;
    ctx.lineWidth = borderWidth;
  
    const drawCorner = (x1: number, y1: number, x2: number, y2: number) => {
      ctx.arcTo(x1, y1, x2, y2, borderRadius);
    };
    const calculations = [
      { x1: x + width / 2, y1: y - height / 2, x2: x + width / 2, y2: y + height / 2 },
      { x1: x + width / 2, y1: y + height / 2, x2: x - width / 2, y2: y + height / 2 },
      { x1: x - width / 2, y1: y + height / 2, x2: x - width / 2, y2: y - height / 2 },
      { x1: x - width / 2, y1: y - height / 2, x2: x + width / 2, y2: y - height / 2 },
    ];
    
    ctx.moveTo(x - width / 2 + borderRadius, y - height / 2);
    calculations.forEach((calculation) => {
      drawCorner(calculation.x1, calculation.y1, calculation.x2, calculation.y2);
    });
  
    ctx.closePath();
    ctx.fill();
    ctx.stroke();
  };
  const nodeCanvasObject = (
    node: NodeObject<NodeObject<GraphNode>>,
    ctx: CanvasRenderingContext2D,
    globalScale: number,
    options: {
      backgroundColor?: string;
      borderColor?: string;
      borderWidth?: number;
      borderRadius?: number;
      circleOptions?: {
        fillColor?: string;
        strokeColor?: string;
        strokeWidth?: number;
        radius?: number;
      };
    } = {}
  ) => {
    const { circleOptions = {} } = options;
    const { fillColor, strokeColor, strokeWidth, radius } = circleOptions;
  
    const label = node.name;
    const fontSize = 8 / globalScale;
  
    const x = node.x ?? 0;
    const y = node.y ?? 0;
  
    // Draw rounded rectangle background
    // drawRoundedRectangle(ctx, x, y, nodeWidth, nodeHeight, borderRadius, backgroundColor, borderColor, borderWidth);
    // Draw circle
    drawCircle(ctx, x, y, { fillColor, strokeColor, strokeWidth, radius });
  
    // Draw text in the center
    ctx.fillStyle = 'white';
    ctx.font = `${fontSize}px Sans-Serif`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText(label, x, y);
  };
  return (
    <div className="bg-gray-800 text-white p-8">
      <h1 className="text-3xl font-bold mb-4">Tree Structure</h1>
      {tree && tree.nodes.length > 0 && (
        <div ref={graphContainerRef} className="h-full w-full bg-gray-900 p-4 rounded-lg shadow-lg">
          <NoSSRForceGraph2D
            linkDirectionalParticles={6}
            nodeAutoColorBy="node_type"
            graphData={flattenTree(tree)}
            nodeCanvasObject={nodeCanvasObject}
          />
        </div>
      )}
    </div>
  );
};

export default TreeGraph;
