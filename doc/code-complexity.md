# code-complexity

description: Function has cyclomatic complexity 'current' but allowed no more than maxcompl.
category: Best Practise Rules
## options

### 0

description: severityDescription
default: DEFAULT_SEVERITY
### 1

description: Maximum allowed cyclomatic complexity
default: DEFAULT_COMPLEXITY
## examples

### good

#### 0

description: Low code complexity
code: require('../../../test/fixtures/best-practises/code-complexity-low')
### bad

#### 0

description: High code complexity
code: require('../../../test/fixtures/best-practises/code-complexity-high')
