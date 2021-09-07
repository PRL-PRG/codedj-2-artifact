# Model report for file:///tmp/top-repos-quality-repos-wbpo3563/arc-dao-stack.git HEAD 20eb4106035eb5b1fc17964dbb3f8f2dd8bb5371

### Dump

```json
{'created_at': '2021-09-01 02:06:49',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '21.3 kB',
 'tags': [],
 'uuid': 'cfb19ebb-2ed5-48fa-b247-4f0d4c9a309e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-wbpo3563/arc-dao-stack.git 20eb4106035eb5b1fc17964dbb3f8f2dd8bb5371

# javascript
135 rules, avg.len. 7.2
## train
PPCR: 0.966528
### report
macro
{'f1-score': 0.5588205261223803,
 'precision': 0.620620454313714,
 'recall': 0.5242121943238434,
 'support': 47010}
micro
{'f1-score': 0.9486066794299085,
 'precision': 0.9486066794299085,
 'recall': 0.9486066794299085,
 'support': 47010}
weighted
{'f1-score': 0.9444350954616642,
 'precision': 0.9441397969129786,
 'recall': 0.9486066794299085,
 'support': 47010}
### report_full
macro
{'f1-score': 0.5241933228792972,
 'precision': 0.620620454313714,
 'recall': 0.4699951611640092,
 'support': 48638}
micro
{'f1-score': 0.9324606891937102,
 'precision': 0.9486066794299085,
 'recall': 0.9168551338459641,
 'support': 48638}
weighted
{'f1-score': 0.922238068768822,
 'precision': 0.934232401212975,
 'recall': 0.9168551338459641,
 'support': 48638}
## test
PPCR: 0.959039
### report
macro
{'f1-score': 0.559576886007721,
 'precision': 0.6095157992300794,
 'recall': 0.5276202640870074,
 'support': 11941}
micro
{'f1-score': 0.9390335817770706,
 'precision': 0.9390335817770706,
 'recall': 0.9390335817770706,
 'support': 11941}
weighted
{'f1-score': 0.933714267733023,
 'precision': 0.9322476317110594,
 'recall': 0.9390335817770706,
 'support': 11941}
### report_full
macro
{'f1-score': 0.5155394467894787,
 'precision': 0.6095157992300794,
 'recall': 0.4632657181162128,
 'support': 12451}
micro
{'f1-score': 0.9193998032141686,
 'precision': 0.9390335817770706,
 'recall': 0.900570235322464,
 'support': 12451}
weighted
{'f1-score': 0.9065956947975974,
 'precision': 0.9218131343240487,
 'recall': 0.900570235322464,
 'support': 12451}
```

## javascript
### Summary
82 rules, avg.len. 6.8

| | |
|-|-|
|Min support|137|
|Max support|24365|
|Min confidence|0.9215213656425476|
|Max confidence|0.999476432800293|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.958. Support: 2392.` |
| 2 | `  -1.reserved = ;<br>	∧ +1.length ≤ 2<br>	∧ +4.reserved = ,<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.950. Support: 169.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles in {ARGUMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.940. Support: 158.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = await<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 808.` |
| 5 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, await}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>⇒ y = ␣<br>Confidence: 0.990. Support: 834.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, await}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = AssignmentExpression<br>⇒ y = ␣<br>Confidence: 0.939. Support: 535.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, await}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^2.internal_type not in {AwaitExpression}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 537.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = async<br>⇒ y = ␣<br>Confidence: 0.998. Support: 206.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = async<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 226.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.944. Support: 169.` |
| 11 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;, async, await, {}<br>	∧ +1.reserved not in {=, {}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.982. Support: 138.` |
| 12 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, async, await, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, async, {}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 23782.` |
| 13 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, await}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>⇒ y = ␣<br>Confidence: 0.989. Support: 841.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, await}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = async<br>⇒ y = ␣<br>Confidence: 0.997. Support: 197.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, async, await, {}<br>	∧ -2.diff_col ≥ 34<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, async, {}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 784.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, async, await, {}<br>	∧ -2.diff_col ≤ 33<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, async, {}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 24307.` |
| 17 | `  -1.reserved = ;<br>	∧ +1.roles in {CALLEE}<br>	∧ +1.length ≤ 2<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.949. Support: 148.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.roles in {STRING}<br>	∧ +3.roles in {ARGUMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.937. Support: 150.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = await<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 763.` |
| 20 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, await}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved = =<br>⇒ y = ␣<br>Confidence: 0.990. Support: 906.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, await}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = AssignmentExpression<br>⇒ y = ␣<br>Confidence: 0.945. Support: 535.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = async<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 223.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved = async<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 202.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, async, await, {}<br>	∧ -1.roles in {COMMENT}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 174.` |
| 25 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, async, await, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ +1.reserved not in {=, async, {}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 24119.` |
| 26 | `  -1.reserved = ;<br>	∧ +1.length ≤ 2<br>	∧ +3.roles in {LITERAL}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.928. Support: 146.` |
| 27 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, async, await, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, async, {}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 24214.` |
| 28 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 2<br>	∧ +5.reserved = async<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.941. Support: 161.` |
| 29 | `  -1.reserved not in {;}<br>	∧ -3.reserved = ;<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 726.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 20009.` |
| 31 | `  -1.diff_col ≤ 7<br>	∧ -1.reserved not in {;}<br>	∧ -1.length ≥ 3<br>	∧ +1.length ≥ 3<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 955.` |
| 32 | `  -1.reserved not in {,, ;}<br>	∧ -1.length ≤ 2<br>	∧ -2.diff_offset ≤ 4<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 3<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.957. Support: 336.` |
| 33 | `  -1.reserved not in {;, {}<br>	∧ -1.length ≤ 2<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.diff_col ≥ 13<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 3<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 356.` |
| 34 | `  -1.reserved not in {;}<br>	∧ -1.roles in {STRING}<br>	∧ -3.length ≤ 3<br>	∧ +1.length ≤ 2<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.942. Support: 300.` |
| 35 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 2<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 389.` |
| 36 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 2<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 366.` |
| 37 | `  -1.reserved = async<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 2<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 154.` |
| 38 | `  -1.reserved not in {;, async}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 2<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 4772.` |
| 39 | `  ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.955. Support: 2451.` |
| 40 | `  -1.reserved = ;<br>	∧ +1.roles in {CALL}<br>	∧ +1.length ≤ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ⏎⏎<br>Confidence: 0.978. Support: 162.` |
| 41 | `  -1.reserved = await<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 777.` |
| 42 | `  -1.reserved not in {;, await}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>⇒ y = ␣<br>Confidence: 0.988. Support: 875.` |
| 43 | `  -1.reserved not in {;, await}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = AssignmentExpression<br>⇒ y = ␣<br>Confidence: 0.949. Support: 543.` |
| 44 | `  -1.reserved = async<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 230.` |
| 45 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = async<br>⇒ y = ␣<br>Confidence: 0.998. Support: 215.` |
| 46 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;, async, await, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression, VariableDeclarator}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 157.` |
| 47 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, async, await, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, async, {}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression, VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 24091.` |
| 48 | `  -1.reserved = ;<br>	∧ +1.roles in {CALLEE}<br>	∧ +1.length ≤ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ⏎⏎<br>Confidence: 0.964. Support: 182.` |
| 49 | `  -1.reserved not in {,, ;}<br>	∧ +1.roles in {STRING}<br>	∧ +3.roles in {ARGUMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = '<br>Confidence: 0.924. Support: 138.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = await<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 753.` |
| 51 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, await}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved = =<br>⇒ y = ␣<br>Confidence: 0.990. Support: 871.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression, VariableDeclarator}<br>	∧ ^2.internal_type = AwaitExpression<br>⇒ y = ∅<br>Confidence: 0.936. Support: 163.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = async<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 198.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, async, await, {}<br>	∧ -1.roles in {COMMENT}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression, VariableDeclarator}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 149.` |
| 55 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, async, await, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ +1.reserved not in {=, async, {}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression, VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 24298.` |
| 56 | `  -1.reserved = ;<br>	∧ +1.length ≤ 2<br>	∧ +2.reserved = (<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.931. Support: 138.` |
| 57 | `  -1.reserved not in {,, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>	∧ +3.roles in {ARGUMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.931. Support: 153.` |
| 58 | `  -1.reserved = await<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 716.` |
| 59 | `  -1.reserved not in {;, await}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≥ 2<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved = =<br>⇒ y = ␣<br>Confidence: 0.986. Support: 823.` |
| 60 | `  -1.reserved not in {;, await}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = AssignmentExpression<br>⇒ y = ␣<br>Confidence: 0.943. Support: 591.` |
| 61 | `  -1.reserved = async<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 223.` |
| 62 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = async<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 206.` |
| 63 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;, async, await, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.978. Support: 157.` |
| 64 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, async, await, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {=, async, {}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 23746.` |
| 65 | `  -1.reserved = ;<br>	∧ +1.length ≤ 2<br>	∧ +3.roles in {ARGUMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ⏎⏎<br>Confidence: 0.948. Support: 144.` |
| 66 | `  -1.reserved not in {,, ;}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles in {ARGUMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = '<br>Confidence: 0.942. Support: 165.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = await<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 719.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, await}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>⇒ y = ␣<br>Confidence: 0.983. Support: 861.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression, VariableDeclarator}<br>	∧ ^2.internal_type = AwaitExpression<br>⇒ y = ∅<br>Confidence: 0.953. Support: 137.` |
| 70 | `  •••start_col ≥ 13<br>	∧ -1.diff_col ≤ 27<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, async, await, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, async, {}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression, VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 24365.` |
| 71 | `  -1.reserved = ;<br>	∧ +1.roles in {CALL}<br>	∧ +1.length ≤ 2<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.946. Support: 175.` |
| 72 | `  -1.reserved not in {;}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type = Identifier<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 761.` |
| 73 | `  -1.reserved not in {;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {Identifier}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 560.` |
| 74 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = )<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 151.` |
| 75 | `  -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {ASSIGNMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 158.` |
| 76 | `  -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {ASSIGNMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 19056.` |
| 77 | `  -1.reserved not in {,, ;}<br>	∧ -1.length ≤ 2<br>	∧ -2.length ≤ 3<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 3<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.943. Support: 340.` |
| 78 | `  -1.reserved not in {;}<br>	∧ -1.length ≤ 2<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ +1.length ≥ 3<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 451.` |
| 79 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ -3.length ≤ 2<br>	∧ +1.length ≤ 2<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.931. Support: 328.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 2<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 386.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 2<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 371.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -5.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 2<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 4634.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.7682926829268295, "max_conf": 0.999476432800293, "max_support": 24365, "min_conf": 0.9215213656425476, "min_support": 137, "num_rules": 82}}
```
</details>
