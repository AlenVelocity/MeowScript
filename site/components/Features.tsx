import React from 'react'

const features = [
  {
    name: 'Cat-inspired Keywords',
    description: 'From purrhaps instead of if to meowtually instead of else, Meowscript adds a whole new level of catness to your programs.',
    icon: '🐈'
  },
  {
    name: 'Paw-some Syntax',
    description:
      'While being not easy to read, the syntax makes sure you have a Paw-some time writing code.',
    icon: '😼'
  },
  {
    name: 'Fur-midable Performance',
    description: 'Meowscript may be cute, but it\'s no kitten when it comes to performance. The language is designed to be fast, well faster than Python atleast.',
    icon: '🙀'
  }
]

const Features = () => {
  return (
    <div className="py-12 bg-bg">
      <div className="max-w-xl mx-auto px-4 sm:px-6 lg:max-w-7xl lg:px-8">
        <dl className="space-y-10 lg:space-y-0 lg:grid lg:grid-cols-3 lg:gap-8">
          {features.map((feature, i) => (
            <div key={feature.name}>
              <dt>
                <div className="flex items-center justify-center h-12 w-12 rounded-md bg-[#2E334E] text-white">
                  {feature.icon}
                </div>
                <div className="mt-5 text-lg leading-6 font-medium text-gray-50">
                  <div className="flex flex-row has-tooltip">
                    {feature.name}{' '}
                  </div>
                </div>
              </dt>
              <dd className="mt-2 text-base text-gray-300">
                {feature.description}
              </dd>
            </div>
          ))}
        </dl>
      </div>
    </div>
  )
}

export default Features
