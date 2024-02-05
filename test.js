// should handle static imports

// default
import x from 'default'
// namespaced
import * as y from 'namespaced'
// named
import { test } from 'named'
// unused
import 'polyfills'

// should handle duplicates
import a from 'duplicate'
import a from 'duplicate'

// should handle dynamic imports
import('dynamic')

// will panic on non-trivial dynamic imports
import('test' + 'test')
const dynamicName = 'dynamicName'
import(dynamicName)

// should handle CJS requires
require('require')

// should panic on non-trivial CJS requires
require('test' + 'test')
require(dynamicName)


