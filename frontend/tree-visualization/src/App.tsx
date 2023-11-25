import React, { useState, useEffect } from 'react';
import axios from 'axios';

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

  const renderTree = (node: TreeNode) => {
    console.log("node", node)
    return <>
      <li key={node.id}>{node.name}</li>
      {node.children && node.children.length > 0 && (
        <ul>{node.children.map((child) => renderTree(child))}</ul>
      )}
    </>
  };

  return (
    <React.StrictMode>
    <div className="App">
      <h1>Tree Structure</h1>
      {tree && tree.nodes.length > 0 && <ul>{tree.nodes.map((element) => renderTree(element))}</ul>}
    </div>
    </React.StrictMode>
  );
}

export default App;
