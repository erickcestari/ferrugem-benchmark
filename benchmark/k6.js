import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate } from 'k6/metrics';

export let errorRate = new Rate('errors');

export let options = {
  stages: [
    { duration: '5s', target: 100 },
    { duration: '5s', target: 1000 },
    { duration: '5s', target: 100 },
  ],
  thresholds: {
    errors: ['rate<0.01'], // Error rate should be less than 1%
    http_req_duration: ['p(95)<500'], // 95% of requests should be below 500ms
  },
};

export default function () {
  let res = http.get('http://127.0.0.1:9999');
  let success = check(res, {
    'status is 200': (r) => r.status === 200,
    'response time is less than 500ms': (r) => r.timings.duration < 500,
  });
  errorRate.add(!success);
  sleep(1);
}