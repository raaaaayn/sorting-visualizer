FROM --platform=$BUILDPLATFORM node:alpine AS frontend-buildstep
COPY ui/package.json ./
COPY ui/package-lock.json ./
RUN npm i
COPY ui ./
RUN npm run build

FROM arm32v7/nginx:alpine
COPY --from=frontend-buildstep /dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
