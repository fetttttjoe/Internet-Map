import React, { useState, useEffect } from 'react';
import axios from 'axios';
import ForceGraph2D from 'react-force-graph-2d';

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

interface GraphNode {
  id: number;
  name: string;
  node_type: string;
}

interface GraphLink {
  source: number;
  target: number;
}

function App() {
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
        const newNode: GraphNode = { id: node.id, name: node.name, node_type: node.node_type };
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
  return (
    <React.StrictMode>
      <div className="App">
        <h1>Tree Structure</h1>
        {tree && tree.nodes.length > 0 && (
          <ForceGraph2D
          graphData={flattenTree(tree)} // Pass the entire tree structure
          nodeCanvasObject={(node, ctx, globalScale) => {
              const label = node.name;
              const fontSize = 12 / globalScale;
              ctx.font = `${fontSize}px Sans-Serif`;
              ctx.fillStyle = 'black';
              const x = node.x || 0; // Use 0 if node.x is undefined
              const y = node.y || 0; // Use 0 if node.y is undefined
              ctx.fillText(label,x, y);
            }}
          />
        )}
      </div>
    </React.StrictMode>
  );
}

export default App;
