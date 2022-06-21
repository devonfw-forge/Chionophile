FROM nginx:1.20
RUN ln -sf /dev/null /var/log/nginx/access.log
RUN rm /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/conf.d/default.conf