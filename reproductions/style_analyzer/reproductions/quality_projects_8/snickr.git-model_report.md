# Model report for file:///tmp/top-repos-quality-repos-dzthos0r/snickr.git HEAD 9d4a1df888a47d69d8a6b991e075b4b7e8df646b

### Dump

```json
{'created_at': '2021-08-31 23:12:17',
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
 'size': '20.8 kB',
 'tags': [],
 'uuid': '2c5e8272-fc54-4f86-9a03-17f4de0fe25a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-dzthos0r/snickr.git 9d4a1df888a47d69d8a6b991e075b4b7e8df646b

# javascript
162 rules, avg.len. 8.7
## train
PPCR: 0.968891
### report
macro
{'f1-score': 0.6909797000995853,
 'precision': 0.713890783412557,
 'recall': 0.6713923069882832,
 'support': 29370}
micro
{'f1-score': 0.9153898535921008,
 'precision': 0.9153898535921008,
 'recall': 0.9153898535921008,
 'support': 29370}
weighted
{'f1-score': 0.9097312462656795,
 'precision': 0.9056891461742659,
 'recall': 0.9153898535921008,
 'support': 29370}
### report_full
macro
{'f1-score': 0.6759283603582283,
 'precision': 0.713890783412557,
 'recall': 0.6447820625068865,
 'support': 30313}
micro
{'f1-score': 0.9009265620025803,
 'precision': 0.9153898535921008,
 'recall': 0.8869132055553722,
 'support': 30313}
weighted
{'f1-score': 0.8926735702735077,
 'precision': 0.9016789047422581,
 'recall': 0.8869132055553722,
 'support': 30313}
## test
PPCR: 0.958481
### report
macro
{'f1-score': 0.6903710147373987,
 'precision': 0.7092624066660355,
 'recall': 0.6744106346842068,
 'support': 7318}
micro
{'f1-score': 0.9025690079256627,
 'precision': 0.9025690079256627,
 'recall': 0.9025690079256627,
 'support': 7318}
weighted
{'f1-score': 0.8968628199118654,
 'precision': 0.8929580342404343,
 'recall': 0.9025690079256627,
 'support': 7318}
### report_full
macro
{'f1-score': 0.6719673128449688,
 'precision': 0.7092624066660355,
 'recall': 0.6412359708663458,
 'support': 7635}
micro
{'f1-score': 0.8834347622550658,
 'precision': 0.9025690079256627,
 'recall': 0.8650949574328749,
 'support': 7635}
weighted
{'f1-score': 0.8743695726067683,
 'precision': 0.8867814406760225,
 'recall': 0.8650949574328749,
 'support': 7635}
```

## javascript
### Summary
90 rules, avg.len. 8.6

| | |
|-|-|
|Min support|138|
|Max support|10510|
|Min confidence|0.9213592410087585|
|Max confidence|0.9986945390701294|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>⇒ y = "<br>Confidence: 0.998. Support: 205.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>⇒ y = '<br>Confidence: 0.995. Support: 980.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ -4.length ≥ 2<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.985. Support: 163.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = {<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 166.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 6<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 318.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 354.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {ClassBody}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 358.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {ClassBody}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 10114.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -4.length ≥ 2<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.953. Support: 138.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 173.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 498.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 973.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.roles in {KEY} and not in {LITERAL}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.940. Support: 539.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,}<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 366.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,}<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {EXPRESSION, LITERAL}<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.929. Support: 232.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 347.` |
| 17 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ +1.roles in {INCOMPLETE} and not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.957. Support: 243.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=, {}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {BODY, TYPE}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 276.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 267.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 4<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {File, Program}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 189.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=, >}<br>	∧ ^1.internal_type not in {File, Program}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 9617.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.935. Support: 178.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = File<br>⇒ y = ␣<br>Confidence: 0.952. Support: 341.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {File}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 293.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {File}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 10099.` |
| 26 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>⇒ y = "<br>Confidence: 0.997. Support: 155.` |
| 27 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>⇒ y = '<br>Confidence: 0.994. Support: 1018.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.991. Support: 166.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.980. Support: 908.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 182.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 496.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 1031.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.roles in {INITIALIZATION} and not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.932. Support: 584.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP} and not in {KEY}<br>	∧ ^1.roles not in {DECLARATION, INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 324.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 4<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 421.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 303.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IDENTIFIER} and not in {MAP}<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 343.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 273.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {File, Program}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 217.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, }}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {File, Program}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 309.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, }}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {File, Program}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 9849.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ -4.length ≥ 2<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.976. Support: 144.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.length ≤ 1<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {MODULE} and not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 236.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, MODULE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 383.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -5.diff_line = 0<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {BLOCK} and not in {DECLARATION, MODULE}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 515.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {EXPRESSION} and not in {BLOCK, DECLARATION, MODULE, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 193.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {, }}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, MODULE, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 9565.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = :<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 528.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {SCOPE} and not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 178.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,}<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ⏎<br>Confidence: 0.944. Support: 225.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,}<br>	∧ +1.reserved not in {import}<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 360.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 393.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 344.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {INCOMPLETE} and not in {FILE}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 235.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ -5.diff_line ≥ 2<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {BODY} and not in {FILE}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 182.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BODY, FILE}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 356.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 4<br>	∧ +2.reserved = ><br>	∧ ^1.roles not in {DECLARATION, FILE, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {BODY, FILE}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 204.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=, >}<br>	∧ ^1.roles not in {DECLARATION, FILE, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {BODY, FILE}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 9555.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.945. Support: 154.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.987. Support: 834.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 377.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 301.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {IMPORT} and not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 242.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 11<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = ClassBody<br>⇒ y = ␣<br>Confidence: 0.981. Support: 233.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {ClassBody}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.991. Support: 171.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, FILE, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {ClassBody}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 10456.` |
| 67 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.997. Support: 171.` |
| 68 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 1008.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.938. Support: 591.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 349.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 313.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {BLOCK} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 224.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_line ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.roles in {BODY} and not in {FILE}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 192.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {ClassBody}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, FILE, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {BODY, FILE}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 10327.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.990. Support: 148.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.982. Support: 843.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 328.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 314.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {IMPORT} and not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 245.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 247.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, FILE, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {BODY, FILE}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 10455.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = "<br>Confidence: 0.983. Support: 148.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = '<br>Confidence: 0.970. Support: 846.` |
| 84 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {BLOCK} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 237.` |
| 85 | `  •••start_col ≥ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.internal_type = File<br>⇒ y = ⏎<br>Confidence: 0.956. Support: 236.` |
| 86 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 8<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.internal_type = File<br>⇒ y = ␣<br>Confidence: 0.975. Support: 381.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 333.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 341.` |
| 89 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {File}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 150.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {File}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 10510.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.6, "max_conf": 0.9986945390701294, "max_support": 10510, "min_conf": 0.9213592410087585, "min_support": 138, "num_rules": 90}}
```
</details>
