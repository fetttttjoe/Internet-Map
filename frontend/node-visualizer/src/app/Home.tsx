// pages/index.tsx
import React from 'react'

import TreeGraph from './components/TreeGraph'

const Home: React.FC = () => {
  return (
    <div className={`flex items-center justify-center bg-[#23395B] p-2`}>
      <div
        className={`rounded-lgp-8 h-full w-full max-w-screen-md bg-[#8EA8C3] shadow-lg`}
      >
        <h1 className={`m-2 mb-6 text-4xl font-bold text-white`}>
          Tree Structure
        </h1>
        <TreeGraph />
      </div>
    </div>
  )
}

export default Home
