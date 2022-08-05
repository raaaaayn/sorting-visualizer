FROM --platform=$BUILDPLATFORM node:alpine AS frontend-buildstep
COPY ui/package.json ./
COPY ui/package-lock.json ./
RUN npm i
COPY ui ./
RUN npm run build

FROM nginx
COPY --from=frontend-buildstep /build /usr/share/nginx/html
