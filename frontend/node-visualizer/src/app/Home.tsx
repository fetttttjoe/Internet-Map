// pages/index.tsx
import React from 'react'

import TreeGraph from './components/TreeGraph'

const Home: React.FC = () => {
  return (
    <div className={`flex items-center justify-center bg-[#c4cad3] p-4`}>
      <div className={` h-full w-full bg-[#23395B] shadow-lg`}>
        <h1 className={`m-2 mb-6 text-4xl font-bold text-white`}>
          Tree Structure
        </h1>
        <TreeGraph />
      </div>
    </div>
  )
}

export default Home
