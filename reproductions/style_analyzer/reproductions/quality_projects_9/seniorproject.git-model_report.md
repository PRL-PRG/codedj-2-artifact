# Model report for file:///tmp/top-repos-quality-repos-6irnugou/seniorproject.git HEAD 30f18fe29fad80e91915a04d8157b4d1496c8d9b

### Dump

```json
{'created_at': '2021-08-31 22:29:54',
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
 'size': '18.3 kB',
 'tags': [],
 'uuid': '2b96c1fa-63e9-4507-84f1-288eef5734dc',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-6irnugou/seniorproject.git 30f18fe29fad80e91915a04d8157b4d1496c8d9b

# javascript
92 rules, avg.len. 6.9
## train
PPCR: 0.906101
### report
macro
{'f1-score': 0.40325341758809213,
 'precision': 0.43563645032158316,
 'recall': 0.38997615067647184,
 'support': 14793}
micro
{'f1-score': 0.8787264246603123,
 'precision': 0.8787264246603123,
 'recall': 0.8787264246603123,
 'support': 14793}
weighted
{'f1-score': 0.8580288083692938,
 'precision': 0.8451774178024533,
 'recall': 0.8787264246603123,
 'support': 14793}
### report_full
macro
{'f1-score': 0.36167582968822454,
 'precision': 0.43563645032158316,
 'recall': 0.3455300959795947,
 'support': 16326}
micro
{'f1-score': 0.8354381567531091,
 'precision': 0.8787264246603123,
 'recall': 0.7962146269753767,
 'support': 16326}
weighted
{'f1-score': 0.7961125825763982,
 'precision': 0.8273344133963063,
 'recall': 0.7962146269753767,
 'support': 16326}
## test
PPCR: 0.894363
### report
macro
{'f1-score': 0.4027721341978665,
 'precision': 0.4196388310912512,
 'recall': 0.39410037001032416,
 'support': 2269}
micro
{'f1-score': 0.8479506390480387,
 'precision': 0.8479506390480388,
 'recall': 0.8479506390480388,
 'support': 2269}
weighted
{'f1-score': 0.8214734953423593,
 'precision': 0.8029855346513964,
 'recall': 0.8479506390480388,
 'support': 2269}
### report_full
macro
{'f1-score': 0.3696257128038686,
 'precision': 0.4196388310912512,
 'recall': 0.3509501859920398,
 'support': 2537}
micro
{'f1-score': 0.8006658343736995,
 'precision': 0.8479506390480388,
 'recall': 0.7583760346866377,
 'support': 2537}
weighted
{'f1-score': 0.7556383673887995,
 'precision': 0.775695745943826,
 'recall': 0.7583760346866377,
 'support': 2537}
```

## javascript
### Summary
41 rules, avg.len. 6.8

| | |
|-|-|
|Min support|141|
|Max support|6328|
|Min confidence|0.9209336042404175|
|Max confidence|0.9991055727005005|

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
| 1 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 232.` |
| 2 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=, >}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 6029.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.length ≥ 3<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 372.` |
| 4 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {MAP}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.roles not in {KEY, LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 6328.` |
| 5 | `  -1.diff_offset ≥ 3<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 141.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.988. Support: 722.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = .<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 539.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {.}<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 259.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, JSXElement}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 170.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 206.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {.}<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 265.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 180.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, IF}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 4863.` |
| 14 | `  -1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.987. Support: 679.` |
| 15 | `  -1.reserved = .<br>	∧ -1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 559.` |
| 16 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {.}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 302.` |
| 17 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, JSXElement}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 170.` |
| 18 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 228.` |
| 19 | `  -1.roles not in {STRING}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {BINARY, DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 4632.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 229.` |
| 21 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≥ 5<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved = ><br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 199.` |
| 22 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {MAP}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=, >}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 6181.` |
| 23 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {MAP}<br>	∧ -4.diff_offset ≤ 4<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 218.` |
| 24 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.996. Support: 672.` |
| 25 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.999. Support: 547.` |
| 26 | `  -1.roles not in {STRING}<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 237.` |
| 27 | `  -1.roles not in {STRING}<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.roles in {COMMENT} and not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 225.` |
| 28 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 179.` |
| 29 | `  -1.diff_offset ≥ 2<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.roles not in {COMMENT, LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression, ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 582.` |
| 30 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 219.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {.}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 233.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 226.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 4820.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.998. Support: 697.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 532.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 245.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.roles in {COMMENT} and not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 215.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 156.` |
| 39 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.roles not in {COMMENT, LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 290.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 224.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 4880.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.7560975609756095, "max_conf": 0.9991055727005005, "max_support": 6328, "min_conf": 0.9209336042404175, "min_support": 141, "num_rules": 41}}
```
</details>
